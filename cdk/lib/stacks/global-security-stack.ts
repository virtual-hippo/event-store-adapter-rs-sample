import { Stack, StackProps } from "aws-cdk-lib";
import { Construct } from "constructs";

import { Hasher } from "../constructs";

export class GlobalSecurityStack extends Stack {
  readonly hasherFnArnParameterName: string;

  constructor(scope: Construct, id: string, props: StackProps) {
    super(scope, id, props);

    const hasher = new Hasher(this, "Hasher");

    this.hasherFnArnParameterName = hasher.hasherFnArnParameterName;
  }
}
