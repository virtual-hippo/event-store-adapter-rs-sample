import * as path from "path";

import {
  CfnOutput,
  aws_cloudfront as cloudfront,
  aws_lambda as lambda,
  aws_cloudfront_origins as origins,
} from "aws-cdk-lib";
import { Construct } from "constructs";

export interface CdnProps {
  readonly lambdaFunctionUrl: lambda.IFunctionUrl;
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
      },
      enableLogging: false,
    });

    new CfnOutput(this, "ApiEndpoint", {
      value: `https://${distribution.distributionDomainName}`,
      description: "API Endpoint URL",
    });
  }
}
