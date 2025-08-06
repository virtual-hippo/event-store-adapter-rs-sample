import * as cdk from "aws-cdk-lib";
import { Template } from "aws-cdk-lib/assertions";
import { APP_PARAMETERS } from "../lib/parameters";
import { EventStoreAdapterRsSampleStage } from "../lib/stages";

export const serializer = {
  test: (val: unknown) => typeof val === "string",
  serialize: (val: string) => {
    return `"${val
      // cloudfront function の簡易認証に使う authString をダミー値に置き換え
      .replace(/const authString = (.+);/, "const authString = REPLACED;")
      // PhysicalResourceId の unix timestamp をダミー値に置き換え
      .replace(/(PhysicalResourceId-[0-9]{13})/, "PhysicalResourceId-REPLACED")
      // Asset hash をダミー値に置き換え
      .replace(/([A-Fa-f0-9]{64}.zip)/, "HASH_REPLACED.zip")
      // Construct address をダミー値に置き換え
      .replace(/[a-f0-9]{42}/, "[CONSTRUCT_ADDR_REPLACED]")}"`;
  },
};

test("Snapshot test", () => {
  const app = new cdk.App();

  const stage = new EventStoreAdapterRsSampleStage(app, "Dev01", {
    envType: "Dev01",
    appParameter: APP_PARAMETERS.Dev01,
  });

  const template = Template.fromStack(stage.eventStoreAdapterRsSampleStack);

  expect.addSnapshotSerializer(serializer);
  expect(template.toJSON()).toMatchSnapshot();
});
