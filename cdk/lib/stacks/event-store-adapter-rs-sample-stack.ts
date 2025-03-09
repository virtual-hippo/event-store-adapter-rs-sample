import * as path from "path";

import { CfnOutput, Stack, StackProps } from "aws-cdk-lib";
import { RustFunction } from "cargo-lambda-cdk";
import { Construct } from "constructs";

import { AppParameter } from "../parameters";

export interface EventStoreAdapterRsSampleStackProps extends StackProps {
  readonly appParameter: AppParameter;
}

export class EventStoreAdapterRsSampleStack extends Stack {
  constructor(scope: Construct, id: string, props?: EventStoreAdapterRsSampleStackProps) {
    super(scope, id, props);

    const fn = new RustFunction(this, "HelloFunction", {
      manifestPath: path.join(__dirname, "../../../", "aws-lambda-functions", "Cargo.toml"),
      binaryName: "hello",
    });

    const fnUrl = fn.addFunctionUrl();

    new CfnOutput(this, "HelloFunctionUrl", { value: fnUrl.url });
  }
}
