// eslint.config.mjs
import eslint from "@eslint/js";
import eslintCdkPlugin from "eslint-cdk-plugin";
import { globalIgnores } from "eslint/config";
import tsEslint from "typescript-eslint";

// https://eslint-cdk-plugin.dev/ja/getting-started/
export default [
  eslint.configs.recommended,
  ...tsEslint.configs.recommended,
  globalIgnores(["**/node_modules/**/*", "**/cdk.out/**/*", "**/*.js", "**/*.d.ts"]),
  {
    files: ["lib/**/*.ts", "bin/*.ts"],
    languageOptions: {
      parserOptions: {
        projectService: true,
        project: "./tsconfig.json",
      },
    },
    // ✅ Add plugins
    plugins: {
      cdk: eslintCdkPlugin,
    },
    // ✅ Add rules (use recommended rules)
    rules: {
      ...eslintCdkPlugin.configs.recommended.rules,
    },
  },
];
