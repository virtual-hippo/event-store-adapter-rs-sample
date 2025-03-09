/**

Cline + Bedrock Claude 3.7 Sonet で生成.
与えたプロンプトは以下.

```md
# Task
アジャイル開発におけるプロダクトバックログのドメインモデリングをしてください．

複雑になりすぎないようにしてください．

# Context
プログラミングの勉強のためにサンプルアプリケーションを実装します．
その題材が欲しいです．
```

 */
pub mod acceptance_criterion;
pub mod backlog_item;
pub mod epic;
pub mod product_backlog;
pub mod sprint;
pub mod types;
