import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { AppParameter, EnvType } from "../parameters";
import { EventStoreAdapterRsSampleStack, GlobalSecurityStack } from "../stacks";

export interface EventStoreAdapterRsSampleStageProps extends cdk.StageProps {
  readonly envType: EnvType;
  readonly appParameter: AppParameter;
}

export class EventStoreAdapterRsSampleStage extends cdk.Stage {
  // for test
  readonly eventStoreAdapterRsSampleStack: EventStoreAdapterRsSampleStack;
  readonly globalSecurityStack: GlobalSecurityStack;

  constructor(scope: Construct, id: string, props: EventStoreAdapterRsSampleStageProps) {
    super(scope, id, props);

    const globalSecurityStack = new GlobalSecurityStack(this, "GlobalSecurity", {
      env: {
        account: props.appParameter.env.account || process.env.CDK_DEFAULT_ACCOUNT,
        region: "us-east-1",
      },
    });

    const eventStoreAdapterRsSampleStack = new EventStoreAdapterRsSampleStack(this, "EventStoreAdapterRsSample", {
      // stack props
      env: {
        account: props.appParameter.env.account || process.env.CDK_DEFAULT_ACCOUNT,
        region: props.appParameter.env.region || process.env.CDK_DEFAULT_ACCOUNT,
      },
      tags: {
        Environment: props.envType,
        SysName: "event-store-adapter-rs-sample",
      },
      // custome props
      envType: props.envType,
      appParameter: props.appParameter,
      contentsHashCalculatorFnArnParameterName: globalSecurityStack.contentsHashCalculatorFnArnParameterName,
    });

    eventStoreAdapterRsSampleStack.addDependency(globalSecurityStack);

    this.globalSecurityStack = globalSecurityStack;
    this.eventStoreAdapterRsSampleStack = eventStoreAdapterRsSampleStack;
  }
}
