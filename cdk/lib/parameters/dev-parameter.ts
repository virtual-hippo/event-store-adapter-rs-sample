import { AppParameter } from "./app-parameter";

export const DEV_PARAMETER: AppParameter = {
  env: {
    // 公開リポジトリのためアカウント ID は明示的に記載しない
    // account: "123456789012",
    region: "ap-northeast-1",
  },
  appFunctionParameter: {
    apiParameter: {
      // 整える
      apiAllowOrigins: ["http://localhost:3000", "http://localhost:5173", "http://localhost:8080"],
    },
  },
} as const;
