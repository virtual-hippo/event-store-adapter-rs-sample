# プロジェクト/Backlog管理アプリ ドメインレイヤ設計

このドキュメントは、**ドメイン駆動設計（DDD）** に基づく「プロジェクト管理アプリ」のドメインレイヤ設計をまとめたものです。  
本アプリケーションのユースケース（アプリケーションサービス）は別途定義し、本ドキュメントではドメインレイヤに関する情報のみを記載します。

---

## 目次

- [1. ユビキタス言語](#1-ユビキタス言語)
- [2. ドメインモデル図](#2-ドメインモデル図)
- [3. エンティティ（Entity）](#3-エンティティentity)
- [4. 値オブジェクト（Value Object）](#4-値オブジェクトvalue-object)
- [5. 集約（Aggregate）](#5-集約aggregate)
- [6. ドメインイベント（Domain Event）](#6-ドメインイベントdomain-event)

---

## 1. ユビキタス言語

本アプリケーションの開発において、以下のドメインに関わる用語をすべての関係者（開発者・業務担当者・ユーザー）が共通の意味で使用します。

- **プロジェクト（Project）**  
  複数のメンバーで構成され、イテレーションやタスクを管理する単位。

- **イテレーション（Iteration）**  
  プロジェクト内で定める短期間の作業サイクル。スプリントとも呼ぶ場合あり。

- **タスク（Task）**  
  イテレーションやプロジェクトに紐付く作業単位。担当者や優先度、状態などを持つ。

- **メンバー（Member）**  
  プロジェクトに参加するユーザー。役割（ロール）を持つ。

- **ユーザー（User）**  
  システムに登録された個人。複数プロジェクトに所属可能。

- **コメント（Comment）**  
  タスクに紐付くメッセージ。ユーザーが作成し、時系列で管理する。

- **タグ（Tag）**  
  タスクに付与するラベル。分類や検索に使用。

- **期間（Period）**  
  開始日と終了日からなる日付のペア。

- **状態（Status）**  
  プロジェクト、イテレーション、タスクなどの進行状況を示す値。

- **優先度（Priority）**  
  タスクの重要度を示す値。Low/Medium/High など。

- **役割（Role）**  
  プロジェクトにおけるメンバーの権限や立場。Admin, Member など。

---

## 2. ドメインモデル図

省略

---

## 3. エンティティ（Entity）

### Projectエンティティ

- `project_id: ProjectId` … プロジェクトを一意に識別するID（値オブジェクト）
- `project_name: ProjectName` … プロジェクト名（値オブジェクト）
- `description: Description` … プロジェクトの説明（値オブジェクト）
- `status: ProjectStatus` … プロジェクトの状態（値オブジェクト）
- `members: Members` … プロジェクトのメンバーリスト（エンティティ）

### Membersエンティティ

- `members: Vec<Member>` … プロジェクトのメンバーリスト（エンティティ）

### Memberエンティティ

- `member_id: MemberId` … プロジェクトメンバーを一意に識別するID（値オブジェクト）
- `role: Role` … プロジェクト内での役割（値オブジェクト）
- `user_id: UserId` … プロジェクトメンバーのユーザーID（値オブジェクト）

### Iterationエンティティ

- `iteration_id: IterationId` … イテレーションを一意に識別するID（値オブジェクト）
- `project_id: ProjectId` … 所属プロジェクトのID（値オブジェクト）
- `name: IterationName` … イテレーション名（値オブジェクト）
- `description: Description` … イテレーションの説明（値オブジェクト）
- `status: IterationStatus` … イテレーションの状態（値オブジェクト）
- `period: Period` … 開始日・終了日（値オブジェクト）

### Taskエンティティ

- `task_id: TaskId` … タスクを一意に識別するID（値オブジェクト）
- `project_id: ProjectId` … 所属プロジェクトのID（値オブジェクト）
- `iteration_id: Option<IterationId>` … 紐付くイテレーションID（値オブジェクト、任意）
- `title: TaskTitle` … タスクタイトル（値オブジェクト）
- `description: Description` … タスク詳細（値オブジェクト）
- `assignee_id: Option<MemberId>` … 担当者ID（値オブジェクト、任意）
- `priority: Priority` … 優先度（値オブジェクト）
- `status: TaskStatus` … ステータス（値オブジェクト）
- `planned_period: Option<Period>` … 予定期間（開始・終了日）（値オブジェクト、任意）
- `actual_period: Option<Period>` … 実績期間（開始・終了日）（値オブジェクト、任意）
- `comments: Vec<Comment>` … コメントリスト（エンティティ）
- `tags: Vec<Tag>` … タグリスト（エンティティ）

### Userエンティティ

- `user_id: UserId` … ユーザーを一意に識別するID（値オブジェクト）
- `username: UserName` … ユーザー名（値オブジェクト）
- `email: Email` … メールアドレス（値オブジェクト）
- `project_memberships: Vec<ProjectMembership>` … 参加プロジェクト情報（エンティティ）

### Commentエンティティ

- `comment_id: CommentId` … コメントID（値オブジェクト）
- `task_id: TaskId` … 対象タスクID（値オブジェクト）
- `author_id: UserId` … コメント投稿者ID（値オブジェクト）
- `body: CommentBody` … コメント本文（値オブジェクト）
- `created_at: DateTime` … 作成日時（値オブジェクト）

### Tagエンティティ

- `tag_id: TagId` … タグID（値オブジェクト）
- `name: TagName` … タグ名（値オブジェクト）

---

## 4. 値オブジェクト（Value Object）

- `ProjectId` … プロジェクトを一意に識別するID
- `ProjectName` … プロジェクト名
- `IterationId` … イテレーションを一意に識別するID
- `IterationName` … イテレーション名
- `TaskId` … タスクを一意に識別するID
- `TaskTitle` … タスクタイトル
- `UserId` … ユーザーを一意に識別するID
- `UserName` … ユーザー名
- `Email` … メールアドレス
- `CommentId` … コメントを一意に識別するID
- `CommentBody` … コメント本文
- `TagId` … タグID
- `TagName` … タグ名
- `Period` … 開始日・終了日のペア
- `Role` … ユーザー権限（Admin、Member など）
- `ProjectStatus` … プロジェクト状態（Planning、InProgress、Completed、Archived など）
- `IterationStatus` … イテレーション状態（Planning、InProgress、Completed、Archived など）
- `TaskStatus` … タスク状態（Todo、InProgress、InReview、Done など）
- `Priority` … 優先度（Low、Medium、High）

---

## 5. 集約（Aggregate）

- **Project集約**
  - ルート: `Project`
  - 子: `Members`（メンバー）

- **Iteration集約**
  - ルート: `Iteration`
  - 子: なし（または`Task`をID参照）

- **Task集約**
  - ルート: `Task`
  - 子: `Comment`, `Tag`

- **User集約**
  - ルート: `User`

---

## 6. ドメインイベント（Domain Event）

- `ProjectCreated` … プロジェクトが作成された
- `MemberRoleChanged` … メンバーの役割が変更された
- `IterationCreated` … イテレーションが作成された
- `TaskCreated` … タスクが作成された
- `TaskAssigned` … タスクに担当者が割り当てられた
- `TaskStatusChanged` … タスクの状態が変更された
- `TaskPriorityChanged` … タスクの優先度が変更された
- `TaskScheduled` … タスクの予定日が設定された
- `CommentAdded` … コメントが追加された
- `UserJoinedProject` … ユーザーがプロジェクトに参加した

---