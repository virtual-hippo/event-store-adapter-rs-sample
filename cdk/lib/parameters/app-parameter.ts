import * as cdk from "aws-cdk-lib";

export interface AppParameter {
  readonly env: cdk.Environment;
  readonly appFunctionParameter: {
    readonly apiParameter: {
      readonly apiAllowOrigins: string[];
    };
  };
}
