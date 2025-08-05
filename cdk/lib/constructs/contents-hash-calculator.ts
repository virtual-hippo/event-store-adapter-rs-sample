import * as path from "path";

import {
  Duration,
  aws_iam as iam,
  aws_lambda as lambda,
  aws_lambda_nodejs as lambda_nodejs,
  aws_ssm as ssm,
} from "aws-cdk-lib";
import { Construct } from "constructs";

export class ContentsHashCalculator extends Construct {
  readonly fnArnParameterName = "/event-store-adapter-rs-sample/contents-hash-calculator-fn-arn" as const;

  constructor(scope: Construct, id: string) {
    super(scope, id);

    const fn = new lambda_nodejs.NodejsFunction(this, "Fn", {
      runtime: lambda.Runtime.NODEJS_22_X,
      entry: path.join(__dirname, "../../", "assets/lambda-at-edge/contents-hash-calculator.ts"),
      handler: "handler",
      timeout: Duration.seconds(5),
      role: new iam.Role(this, "FnRole", {
        assumedBy: new iam.CompositePrincipal(
          new iam.ServicePrincipal("lambda.amazonaws.com"),
          new iam.ServicePrincipal("edgelambda.amazonaws.com"),
        ),
        managedPolicies: [iam.ManagedPolicy.fromAwsManagedPolicyName("service-role/AWSLambdaBasicExecutionRole")],
      }),
    });

    new ssm.StringParameter(this, "FnArnParameter", {
      description: "The ContentsHashCalculatorFn ARN for event-store-adapter-rs-sample",
      parameterName: this.fnArnParameterName,
      stringValue: fn.currentVersion.functionArn,
      tier: ssm.ParameterTier.STANDARD,
    });
  }
}
