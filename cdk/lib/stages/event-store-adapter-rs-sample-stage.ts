import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { AppParameter, EnvType } from "../parameters";
import { EventStoreAdapterRsSampleStack } from "../stacks";

export interface EventStoreAdapterRsSampleStageProps extends cdk.StageProps {
  readonly envType: EnvType;
  readonly appParameter: AppParameter;
}

export class EventStoreAdapterRsSampleStage extends cdk.Stage {
  // for test
  readonly eventStoreAdapterRsSampleStack: EventStoreAdapterRsSampleStack;

  constructor(scope: Construct, id: string, props: EventStoreAdapterRsSampleStageProps) {
    super(scope, id, props);

    const eventStoreAdapterRsSampleStack = new EventStoreAdapterRsSampleStack(this, "EventStoreAdapterRsSample", {
      env: {
        account: props.appParameter.env.account || process.env.CDK_DEFAULT_ACCOUNT,
        region: props.appParameter.env.region || process.env.CDK_DEFAULT_ACCOUNT,
      },
      tags: {
        Environment: props.envType,
        SysName: "event-store-adapter-rs-sample",
      },
      appParameter: props.appParameter,
    });

    this.eventStoreAdapterRsSampleStack = eventStoreAdapterRsSampleStack;
  }
}
