import { Stack, StackProps } from "aws-cdk-lib";
import { Construct } from "constructs";

import { ContentsHashCalculator } from "../constructs";

export class GlobalSecurityStack extends Stack {
  readonly contentsHashCalculatorFnArnParameterName: string;

  constructor(scope: Construct, id: string, props: StackProps) {
    super(scope, id, props);

    const contentsHashCalculator = new ContentsHashCalculator(this, "ContentsHashCalculator");

    this.contentsHashCalculatorFnArnParameterName = contentsHashCalculator.fnArnParameterName;
  }
}
