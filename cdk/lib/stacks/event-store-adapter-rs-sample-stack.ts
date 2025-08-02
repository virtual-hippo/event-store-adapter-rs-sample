import { Stack, StackProps } from "aws-cdk-lib";
import { Construct } from "constructs";

import { AppFunction } from "../constructs";

import { AppParameter } from "../parameters";

export interface EventStoreAdapterRsSampleStackProps extends StackProps {
  readonly appParameter: AppParameter;
}

export class EventStoreAdapterRsSampleStack extends Stack {
  constructor(scope: Construct, id: string, props: EventStoreAdapterRsSampleStackProps) {
    super(scope, id, props);

    new AppFunction(this, "AppFunction", {
      apiParameter: props.appParameter.appFunctionParameter.apiParameter,
    });
  }
}
