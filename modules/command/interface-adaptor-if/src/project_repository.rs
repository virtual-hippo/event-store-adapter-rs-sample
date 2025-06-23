use std::fmt::Debug;

use event_store_adapter_rs::types::{EventStoreReadError, EventStoreWriteError};
use thiserror::Error;

use command_domain::project::*;

#[derive(Debug, Error)]
pub enum ProjectRepositoryError {
    #[error("Failed to store the project: {0:?}")]
    StoreError(Project, EventStoreWriteError),
    #[error("Failed to find the project by id: {0:?}")]
    FindByIdError(ProjectId, EventStoreReadError),
}

/// プロジェクトのリポジトリ。
#[async_trait::async_trait]
pub trait ProjectRepository: Debug + Clone + Sync + Send + 'static {
    /// プロジェクトのイベント及びスナップを保存する。
    ///
    /// # 引数
    /// - `event` - プロジェクトのイベント
    /// - `version` - プロジェクトのバージョン
    /// - `snapshot` - プロジェクトのスナップショット
    ///
    /// # 戻り値
    /// - 成功した場合はOk, 失敗した場合はErrを返す。
    async fn store(&mut self, event: &ProjectEvent, snapshot: &Project) -> Result<(), ProjectRepositoryError>;

    /// 指定したプロジェクトIDに該当するプロジェクトを取得する。
    ///
    /// # 引数
    /// - `id` - プロジェクトID
    ///
    /// # 戻り値
    /// - 取得できた場合はOk(Project), 取得できなかった場合はErrを返す。
    async fn find_by_id(&self, id: &ProjectId) -> Result<Option<Project>, ProjectRepositoryError>;
}
