import { AppParameter } from "./app-parameter";

export const DEV_PARAMETER: AppParameter = {
  env: {
    // 公開リポジトリのためアカウント ID は明示的に記載しない
    // account: "123456789012",
    region: "ap-northeast-1",
  },
} as const;
