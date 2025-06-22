use chrono::{DateTime, Utc};
use event_store_adapter_rs::types::Aggregate;
use serde::{Deserialize, Serialize};

mod member;
mod member_id;
mod member_role;
mod members;
mod project_error;
mod project_events;
mod project_id;
mod project_name;

pub use crate::project::member::Member;
pub use crate::project::member_id::MemberId;
pub use crate::project::member_role::MemberRole;
pub use crate::project::members::Members;
pub use crate::project::project_error::ProjectError;
pub use crate::project::project_events::{
    ProjectEvent, ProjectEventCreatedBody, ProjectEventDeletedBody, ProjectEventMemberAddedBody,
    ProjectEventMemberRemovedBody,
};
pub use crate::project::project_id::ProjectId;
pub use crate::project::project_name::ProjectName;
pub use crate::user::user_id::UserId;

// Serialize, Deserialize はドメインモデルに実装しないようにしたい
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    id: ProjectId,
    deleted: bool,
    name: ProjectName,
    members: Members,
    version: usize,
    seq_nr_counter: usize,
    last_updated_at: DateTime<Utc>,
}

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Aggregate for Project {
    type ID = ProjectId;

    fn id(&self) -> &Self::ID {
        &self.id
    }

    fn seq_nr(&self) -> usize {
        self.seq_nr_counter
    }

    fn version(&self) -> usize {
        self.version
    }

    fn set_version(&mut self, version: usize) {
        self.version = version;
    }

    fn last_updated_at(&self) -> &DateTime<Utc> {
        &self.last_updated_at
    }
}

impl Project {
    pub fn new(name: ProjectName, members: Members, executor_id: UserId) -> (Self, ProjectEvent) {
        let id = ProjectId::new();
        Self::from(id, false, name, members, 0, 1, executor_id)
    }

    pub fn from(
        id: ProjectId,
        deleted: bool,
        name: ProjectName,
        members: Members,
        seq_nr_counter: usize,
        version: usize,
        executor_id: UserId,
    ) -> (Self, ProjectEvent) {
        let now = Utc::now();
        let mut my_self = Self {
            id: id.clone(),
            deleted,
            name: name.clone(),
            members: members.clone(),
            seq_nr_counter,
            version,
            last_updated_at: now,
        };
        my_self.seq_nr_counter += 1;
        let event = ProjectEvent::ProjectCreated(ProjectEventCreatedBody::new(
            id,
            my_self.seq_nr_counter,
            name,
            members,
            executor_id,
            now,
        ));
        (my_self, event)
    }

    fn apply_event(&mut self, event: &ProjectEvent) {
        match event {
            ProjectEvent::ProjectDeleted(body) => {
                self.delete(body.executor_id.clone()).unwrap();
            },
            ProjectEvent::ProjectMemberAdded(body) => {
                self.add_member(
                    body.member.breach_encapsulation_of_id().clone(),
                    body.member.breach_encapsulation_of_user_id().clone(),
                    body.member.breach_encapsulation_of_role().clone(),
                    body.executor_id.clone(),
                )
                .unwrap();
            },
            ProjectEvent::ProjectMemberRemoved(body) => {
                self.remove_member(body.user_id.clone(), body.executor_id.clone()).unwrap();
            },
            _ => {},
        }
    }

    pub fn replay(events: &Vec<ProjectEvent>, snapshot: Project) -> Self {
        log::debug!("event.size = {}", events.len());
        events.iter().fold(snapshot, |mut result, event| {
            log::debug!("Replaying snapshot: {:?}", result);
            log::debug!("Replaying event: {:?}", event);
            result.apply_event(event);
            result
        })
    }

    /// [ProjectName]の参照を返す。
    pub fn name(&self) -> &ProjectName {
        &self.name
    }

    /// [Members]の参照を返す
    pub fn members(&self) -> &Members {
        &self.members
    }

    /// プロジェクトを削除する
    ///
    /// # 引数
    /// - executor_id: 実行者のユーザID
    ///
    /// # 戻り値
    /// - プロジェクトが削除されている場合はエラーを返す。
    /// - 実行者が管理者でない場合はエラーを返す。
    /// - 成功した場合は、ProjectDeletedイベントを返す。
    pub fn delete(&mut self, executor_id: UserId) -> Result<ProjectEvent, ProjectError> {
        if self.deleted {
            return Err(ProjectError::AlreadyDeletedError(self.id.clone()));
        }
        if !self.members.is_administrator(&executor_id) {
            return Err(ProjectError::NotAdministratorError(
                "executor_id".to_string(),
                executor_id,
            ));
        }
        self.deleted = true;
        self.seq_nr_counter += 1;
        let now = Utc::now();
        Ok(ProjectEvent::ProjectDeleted(ProjectEventDeletedBody::new(
            self.id.clone(),
            self.seq_nr_counter,
            executor_id,
            now,
        )))
    }

    /// プロジェクトにメンバーを追加する
    ///
    /// # 引数
    /// - member_id: メンバーID
    /// - user_id: ユーザID
    /// - role: メンバーの役割
    /// - executor_id: 実行者のユーザID
    ///
    /// # 戻り値
    /// - プロジェクトが削除されている場合はエラーを返す。
    /// - 実行者が管理者でない場合はエラーを返す。
    /// - ユーザIDが既にメンバーに設定されている場合はエラーを返す。
    /// - 成功した場合は、ProjectMemberAddedイベントを返す。
    pub fn add_member(
        &mut self,
        member_id: MemberId,
        user_id: UserId,
        role: MemberRole,
        executor_id: UserId,
    ) -> Result<ProjectEvent, ProjectError> {
        if self.deleted {
            return Err(ProjectError::AlreadyDeletedError(self.id.clone()));
        }
        if !self.members.is_administrator(&executor_id) {
            return Err(ProjectError::NotAdministratorError(
                "executor_id".to_string(),
                executor_id,
            ));
        }
        if self.members.is_member(&user_id) {
            return Err(ProjectError::AlreadyMemberError(
                "user_id".to_string(),
                user_id,
            ));
        }
        let member = Member::new(member_id, user_id, role);
        self.members.add_member(member.clone());
        self.seq_nr_counter += 1;
        let now = Utc::now();
        Ok(ProjectEvent::ProjectMemberAdded(
            ProjectEventMemberAddedBody::new(
                self.id.clone(),
                self.seq_nr_counter,
                member,
                executor_id,
                now,
            ),
        ))
    }

    /// プロジェクトからメンバーを削除する
    ///
    /// # 引数
    /// - user_id: ユーザID
    /// - executor_id: 実行者のユーザID
    ///
    /// # 戻り値
    /// - プロジェクトが削除されている場合はエラーを返す。
    /// - 実行者が管理者でない場合はエラーを返す。
    /// - ユーザIDがメンバーに設定されていない場合はエラーを返す。
    /// - 成功した場合は、ProjectMemberRemovedイベントを返す。
    pub fn remove_member(&mut self, user_id: UserId, executor_id: UserId) -> Result<ProjectEvent, ProjectError> {
        if self.deleted {
            return Err(ProjectError::AlreadyDeletedError(self.id.clone()));
        }
        if !self.members.is_administrator(&executor_id) {
            return Err(ProjectError::NotAdministratorError(
                "executor_id".to_string(),
                executor_id,
            ));
        }
        if !self.members.is_member(&user_id) {
            return Err(ProjectError::NotMemberError("user_id".to_string(), user_id));
        }

        self.members.remove_member_by_user_id(&user_id);
        self.seq_nr_counter += 1;
        let now = Utc::now();
        Ok(ProjectEvent::ProjectMemberRemoved(
            ProjectEventMemberRemovedBody::new(
                self.id.clone(),
                self.seq_nr_counter,
                user_id,
                executor_id,
                now,
            ),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_project() {
        let executor_id = UserId::default();
        let user_id = UserId::default();
        let mut members = Members::new();
        members.add_member(Member::new(
            MemberId::new(),
            executor_id.clone(),
            MemberRole::Admin,
        ));

        let (mut project, _) = Project::new(
            ProjectName::new("Test").unwrap(),
            members,
            executor_id.clone(),
        );

        let result = project.delete(user_id.clone());
        assert!(result.is_err());

        let result = project.delete(executor_id.clone());
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_member() {
        let executor_id = UserId::default();
        let user_id = UserId::default();
        let member_id = MemberId::new();
        let mut members = Members::new();
        members.add_member(Member::new(
            MemberId::new(),
            executor_id.clone(),
            MemberRole::Admin,
        ));

        let (mut project, _) = Project::new(
            ProjectName::new("Test").unwrap(),
            members,
            executor_id.clone(),
        );

        let _ = project
            .add_member(
                member_id,
                user_id.clone(),
                MemberRole::Member,
                executor_id.clone(),
            )
            .unwrap();

        assert!(project.members().is_member(&user_id));
    }

    #[test]
    fn test_remove_member() {
        let executor_id = UserId::default();
        let user_id = UserId::default();
        let member_id = MemberId::new();
        let mut members = Members::new();
        members.add_member(Member::new(
            MemberId::new(),
            executor_id.clone(),
            MemberRole::Admin,
        ));

        let (mut project, _) = Project::new(
            ProjectName::new("Test").unwrap(),
            members,
            executor_id.clone(),
        );

        let _ = project
            .add_member(
                member_id,
                user_id.clone(),
                MemberRole::Member,
                executor_id.clone(),
            )
            .unwrap();
        let _ = project.remove_member(user_id.clone(), executor_id.clone()).unwrap();

        assert!(!project.members().is_member(&user_id));
    }

    #[test]
    fn test_to_json() {
        let executor_id = UserId::default();
        let project_name = ProjectName::new("test").unwrap();
        let mut members = Members::new();
        members.add_member(Member::new(
            MemberId::new(),
            executor_id.clone(),
            MemberRole::Admin,
        ));

        let (mut project, _) = Project::new(project_name.clone(), members, executor_id.clone());
        assert_eq!(project.name, project_name);

        let _ = project
            .add_member(
                MemberId::new(),
                UserId::default(),
                MemberRole::Member,
                executor_id.clone(),
            )
            .unwrap();

        let json = serde_json::to_string(&project).unwrap();
        println!("{}", json);
        assert!(!json.is_empty());
    }
}
