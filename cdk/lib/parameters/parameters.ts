import { AppParameter } from "./app-parameter";
import { DEV_PARAMETER } from "./dev-parameter";

export const envs = ["Dev01"] as const;
export type EnvType = (typeof envs)[number];

export const APP_PARAMETERS: Record<EnvType, AppParameter> = {
  Dev01: DEV_PARAMETER,
} as const;
