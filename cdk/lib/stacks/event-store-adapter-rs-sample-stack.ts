import { Stack, StackProps } from "aws-cdk-lib";
import { Construct } from "constructs";

import { AppFunction, Cdn, EventStore } from "../constructs";

import { AppParameter, EnvType } from "../parameters";

export interface EventStoreAdapterRsSampleStackProps extends StackProps {
  readonly appParameter: AppParameter;
  readonly envType: EnvType;
  readonly hasherFnArnParameterName: string;
}

export class EventStoreAdapterRsSampleStack extends Stack {
  constructor(scope: Construct, id: string, props: EventStoreAdapterRsSampleStackProps) {
    super(scope, id, props);

    const journalGsiName = `${props.envType}-journal-aid-index`;
    const snapshotGsiName = `${props.envType}-snapshot-aid-index`;

    const eventStore = new EventStore(this, "EventStore", {
      journalGsiName: journalGsiName,
      snapshotGsiName: snapshotGsiName,
    });

    const appFunction = new AppFunction(this, "AppFunction", {
      apiParameter: props.appParameter.appFunctionParameter.apiParameter,
      journalTable: eventStore.journalTable,
      journalGsiName: journalGsiName,
      snapshotTable: eventStore.snapshotTable,
      snapshotGsiName: snapshotGsiName,
    });

    new Cdn(this, "Cdn", {
      lambdaFunctionUrl: appFunction.writeApiFnUrl,
      hasherFnArnParameterName: props.hasherFnArnParameterName,
    });
  }
}
