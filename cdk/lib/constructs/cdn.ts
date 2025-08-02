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
      },
      enableLogging: false,
    });

    new CfnOutput(this, "ApiEndpoint", {
      value: `https://${distribution.distributionDomainName}`,
      description: "API Endpoint URL",
    });
  }
}
