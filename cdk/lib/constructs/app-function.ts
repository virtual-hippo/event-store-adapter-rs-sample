import * as path from "path";

import { Duration, aws_dynamodb as dynamodb, aws_lambda as lambda } from "aws-cdk-lib";
import { Construct } from "constructs";

import { RustFunction } from "cargo-lambda-cdk";

export interface AppFunctionProps {
  readonly apiParameter: {
    readonly apiAllowOrigins: string[];
  };
  readonly journalTable: dynamodb.ITableV2;
  readonly journalGsiName: string;
  readonly snapshotTable: dynamodb.ITableV2;
  readonly snapshotGsiName: string;
}

export class AppFunction extends Construct {
  /**
   * Write API の Lambda 関数の URL
   */
  readonly writeApiFnUrl: lambda.IFunctionUrl;

  constructor(scope: Construct, id: string, props: AppFunctionProps) {
    super(scope, id);

    {
      const fn = new RustFunction(this, "WriteApiFunction", {
        manifestPath: path.join(__dirname, "../../../", "aws-lambda-functions", "Cargo.toml"),
        binaryName: "write-api",
        // TODO: 整える
        environment: {
          APP__API__ALLOW_ORIGINS: props.apiParameter.apiAllowOrigins.join(","),
          APP__PERSISTENCE__JOURNAL_TABLE_NAME: props.journalTable.tableName,
          APP__PERSISTENCE__JOURNAL_AID_INDEX_NAME: props.journalGsiName,
          APP__PERSISTENCE__SNAPSHOT_TABLE_NAME: props.snapshotTable.tableName,
          APP__PERSISTENCE__SNAPSHOT_AID_INDEX_NAME: props.snapshotGsiName,
          APP__PERSISTENCE__SHARD_COUNT: "64",
          APP__PERSISTENCE__SNAPSHOT_INTERVAL: "10",
          APP__AWS__REGION_NAME: "ap-northeast-1",
        },
      });

      props.journalTable.grantReadWriteData(fn);
      props.snapshotTable.grantReadWriteData(fn);

      const fnUrl = fn.addFunctionUrl({
        // TODO: 整える
        cors: {
          allowedOrigins: [...props.apiParameter.apiAllowOrigins],
          allowedMethods: [lambda.HttpMethod.POST],
          allowedHeaders: ["*"],
          maxAge: Duration.seconds(60),
        },
      });

      this.writeApiFnUrl = fnUrl;
    }
  }
}
