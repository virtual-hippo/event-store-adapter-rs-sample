// eslint.config.mjs
import eslintCdkPlugin from "eslint-cdk-plugin";
import tsEslint from "typescript-eslint";

// https://eslint-cdk-plugin.dev/ja/getting-started/
export default [
  ...tsEslint.configs.recommended,
  {
    files: ["lib/**/*.ts", "bin/*.ts"],
    languageOptions: {
      parserOptions: {
        projectService: true,
        project: "./tsconfig.json",
      },
    },
    ignores: ["**/node_modules/**/*", "**/cdk.out/**/*", "**/*.js", "**/*.d.ts"],
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
