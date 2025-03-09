#!/usr/bin/env node
import * as cdk from "aws-cdk-lib";
import { APP_PARAMETERS } from "../lib/parameters";
import { EventStoreAdapterRsSampleStage } from "../lib/stages";

const app = new cdk.App();
new EventStoreAdapterRsSampleStage(app, "Dev01", {
  envType: "Dev01",
  appParameter: APP_PARAMETERS.Dev01,
});
