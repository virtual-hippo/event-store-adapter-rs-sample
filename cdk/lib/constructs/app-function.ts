import * as path from "path";

import { CfnOutput, Duration } from "aws-cdk-lib";
import * as lambda from "aws-cdk-lib/aws-lambda";
import { Construct } from "constructs";

import { RustFunction } from "cargo-lambda-cdk";

export interface AppFunctionProps {
  readonly apiParameter: {
    readonly apiAllowOrigins: string[];
  };
}

export class AppFunction extends Construct {
  /**
   * Write API の Lambda 関数
   */
  readonly writeApiFn: lambda.IFunction;

  constructor(scope: Construct, id: string, props: AppFunctionProps) {
    super(scope, id);

    {
      const fn = new RustFunction(this, "WriteApiFunction", {
        manifestPath: path.join(__dirname, "../../../", "aws-lambda-functions", "Cargo.toml"),
        binaryName: "write-api",
        // TODO: 整える
        environment: {
          APP__API__ALLOW_ORIGINS: props.apiParameter.apiAllowOrigins.join(","),
          APP__PERSISTENCE__JOURNAL_TABLE_NAME: "journal",
          APP__PERSISTENCE__JOURNAL_AID_INDEX_NAME: "journal-aid-index",
          APP__PERSISTENCE__SNAPSHOT_TABLE_NAME: "snapshot",
          APP__PERSISTENCE__SNAPSHOT_AID_INDEX_NAME: "snapshot-aid-index",
          APP__PERSISTENCE__SHARD_COUNT: "64",
          APP__PERSISTENCE__SNAPSHOT_INTERVAL: "10",
          APP__AWS__REGION_NAME: "ap-northeast-1",
          APP__AWS__ENDPOINT_URL: "x",
          APP__AWS__ACCESS_KEY_ID: "x",
          APP__AWS__SECRET_ACCESS_KEY: "x",
        },
      });

      const fnUrl = fn.addFunctionUrl({
        // TODO: 整える
        cors: {
          allowedOrigins: [...props.apiParameter.apiAllowOrigins],
          allowedMethods: [lambda.HttpMethod.POST],
          allowedHeaders: ["*"],
          maxAge: Duration.seconds(300),
        },
      });

      new CfnOutput(this, "WriteApiFunctionUrl", { value: fnUrl.url });

      this.writeApiFn = fn;
    }
  }
}
