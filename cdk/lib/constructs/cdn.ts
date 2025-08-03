import * as path from "path";

import {
  CfnOutput,
  Stack,
  aws_cloudfront as cloudfront,
  aws_iam as iam,
  aws_lambda as lambda,
  aws_cloudfront_origins as origins,
} from "aws-cdk-lib";
import { AwsCustomResource, AwsCustomResourcePolicy, PhysicalResourceId } from "aws-cdk-lib/custom-resources";
import { Construct } from "constructs";

export interface CdnProps {
  readonly lambdaFunctionUrl: lambda.IFunctionUrl;
  readonly hasherFnArnParameterName: string;
}

export class Cdn extends Construct {
  constructor(scope: Construct, id: string, props: CdnProps) {
    super(scope, id);

    // 簡易的にBasic認証を用意しておく
    const basicAuthenticationFunction = new cloudfront.Function(this, "BasicAuthenticationFunction", {
      code: cloudfront.FunctionCode.fromFile({
        filePath: path.join(__dirname, "../../", "assets/cloudfront-function/basic-authentication.js"),
      }),
      runtime: cloudfront.FunctionRuntime.JS_2_0,
    });

    const distribution = new cloudfront.Distribution(this, "ApiDistribution", {
      comment: "event-store-adapter-rs-sample",
      priceClass: cloudfront.PriceClass.PRICE_CLASS_100,
      defaultBehavior: {
        origin: origins.FunctionUrlOrigin.withOriginAccessControl(props.lambdaFunctionUrl),
        viewerProtocolPolicy: cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
        allowedMethods: cloudfront.AllowedMethods.ALLOW_ALL,
        cachePolicy: cloudfront.CachePolicy.CACHING_DISABLED,
        originRequestPolicy: cloudfront.OriginRequestPolicy.ALL_VIEWER_EXCEPT_HOST_HEADER,
        responseHeadersPolicy: cloudfront.ResponseHeadersPolicy.SECURITY_HEADERS,
        functionAssociations: [
          {
            function: basicAuthenticationFunction,
            eventType: cloudfront.FunctionEventType.VIEWER_REQUEST,
          },
        ],
        edgeLambdas: [
          {
            eventType: cloudfront.LambdaEdgeEventType.ORIGIN_REQUEST,
            functionVersion: lambda.Version.fromVersionArn(
              this,
              "EdgeHasherFnVersion",
              this.getLambdaEdgeArn(props.hasherFnArnParameterName),
            ),
            includeBody: true,
          },
        ],
      },
      // production 環境ではログを有効にする
      // ただし、開発環境ではログを無効にしてコストを抑える
      enableLogging: false,
    });

    new CfnOutput(this, "ApiEndpoint", {
      value: `https://${distribution.distributionDomainName}`,
      description: "API Endpoint URL",
    });
  }

  private getLambdaEdgeArn(hasherFnArnParameterName: string): string {
    const stack = Stack.of(this);

    // コンテンツハッシュ計算用の Lambda 関数の ARN を SSM パラメータから取得するカスタムリソース
    const hasherFnArnParameter = new AwsCustomResource(this, "HasherFnArnParameterCustomResource", {
      policy: AwsCustomResourcePolicy.fromStatements([
        new iam.PolicyStatement({
          effect: iam.Effect.ALLOW,
          actions: ["ssm:GetParameter*"],
          resources: [
            stack.formatArn({
              service: "ssm",
              region: "us-east-1",
              resource: "*",
            }),
          ],
        }),
      ]),
      onUpdate: {
        service: "SSM",
        action: "getParameter",
        parameters: { Name: hasherFnArnParameterName },
        physicalResourceId: PhysicalResourceId.of(`PhysicalResourceId-${Date.now()}`),
        region: "us-east-1",
      },
    });

    return hasherFnArnParameter.getResponseField("Parameter.Value");
  }
}
