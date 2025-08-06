import { RemovalPolicy, aws_dynamodb as dynamodb } from "aws-cdk-lib";
import { Construct } from "constructs";

export interface EventStoreProps {
  readonly journalGsiName: string;
  readonly snapshotGsiName: string;
}

export class EventStore extends Construct {
  readonly journalTable: dynamodb.ITableV2;
  readonly snapshotTable: dynamodb.ITableV2;

  constructor(scope: Construct, id: string, props: EventStoreProps) {
    super(scope, id);

    //
    // Create DynamoDB table for journal
    //
    const journalTable = new dynamodb.TableV2(this, "JournalTable", {
      partitionKey: { name: "pkey", type: dynamodb.AttributeType.STRING },
      sortKey: { name: "skey", type: dynamodb.AttributeType.STRING },
      tableClass: dynamodb.TableClass.STANDARD_INFREQUENT_ACCESS,
      billing: dynamodb.Billing.onDemand(),
      dynamoStream: dynamodb.StreamViewType.NEW_IMAGE,
      removalPolicy: RemovalPolicy.DESTROY,
    });

    journalTable.addGlobalSecondaryIndex({
      indexName: props.journalGsiName,
      partitionKey: { name: "aid", type: dynamodb.AttributeType.STRING },
      sortKey: { name: "seq_nr", type: dynamodb.AttributeType.NUMBER },
      projectionType: dynamodb.ProjectionType.ALL,
    });

    //
    // Create DynamoDB table for snapshots
    //
    const snapshotTable = new dynamodb.TableV2(this, "SnapshotTable", {
      partitionKey: { name: "pkey", type: dynamodb.AttributeType.STRING },
      sortKey: { name: "skey", type: dynamodb.AttributeType.STRING },
      tableClass: dynamodb.TableClass.STANDARD_INFREQUENT_ACCESS,
      billing: dynamodb.Billing.onDemand(),
      removalPolicy: RemovalPolicy.DESTROY,
    });

    snapshotTable.addGlobalSecondaryIndex({
      indexName: props.snapshotGsiName,
      partitionKey: { name: "aid", type: dynamodb.AttributeType.STRING },
      sortKey: { name: "seq_nr", type: dynamodb.AttributeType.NUMBER },
      projectionType: dynamodb.ProjectionType.ALL,
    });

    this.journalTable = journalTable;
    this.snapshotTable = snapshotTable;
  }
}
