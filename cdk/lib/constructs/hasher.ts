import * as path from "path";

import {
  Duration,
  aws_iam as iam,
  aws_lambda as lambda,
  aws_lambda_nodejs as lambda_nodejs,
  aws_ssm as ssm,
} from "aws-cdk-lib";
import { Construct } from "constructs";

export class Hasher extends Construct {
  readonly hasherFnArnParameterName = "/event-store-adapter-rs-sample/edge-hasher-fn-arn" as const;

  constructor(scope: Construct, id: string) {
    super(scope, id);

    const hasherFn = new lambda_nodejs.NodejsFunction(this, "EdgeHasherFn", {
      runtime: lambda.Runtime.NODEJS_22_X,
      entry: path.join(__dirname, "../../", "assets/lambda-at-edge/hasher.ts"),
      handler: "handler",
      timeout: Duration.seconds(5),
      role: new iam.Role(this, "EdgeHasherFnRole", {
        assumedBy: new iam.CompositePrincipal(
          new iam.ServicePrincipal("lambda.amazonaws.com"),
          new iam.ServicePrincipal("edgelambda.amazonaws.com"),
        ),
        managedPolicies: [iam.ManagedPolicy.fromAwsManagedPolicyName("service-role/AWSLambdaBasicExecutionRole")],
      }),
    });

    new ssm.StringParameter(this, "EdgeHasherFnArn", {
      description: "The EdgeHasherFn ARN for event-store-adapter-rs-sample",
      parameterName: this.hasherFnArnParameterName,
      stringValue: hasherFn.currentVersion.functionArn,
      tier: ssm.ParameterTier.STANDARD,
    });
  }
}
