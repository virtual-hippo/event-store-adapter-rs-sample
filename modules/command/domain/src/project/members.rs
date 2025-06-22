use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::project::{Member, MemberId, MemberRole};
use crate::user::user_id::UserId;

/// メンバー集合
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Members {
    members_ids_by_user_id: BTreeMap<String, MemberId>,
    members: BTreeMap<String, Member>,
}

impl Members {
    pub fn new() -> Self {
        Self {
            members_ids_by_user_id: BTreeMap::new(),
            members: BTreeMap::new(),
        }
    }

    /// 管理者かどうかを判定する
    pub fn is_administrator(&self, user_id: &UserId) -> bool {
        self.is_role(user_id, &[MemberRole::Admin])
    }

    /// メンバーかどうかを判定する
    pub fn is_member(&self, user_id: &UserId) -> bool {
        self.is_role(user_id, &[MemberRole::Member, MemberRole::Admin])
    }

    /// ロールを判定する
    pub fn is_role(&self, user_id: &UserId, roles: &[MemberRole]) -> bool {
        if let Some(member_id) = self.members_ids_by_user_id.get(&user_id.to_string()) {
            if let Some(member) = self.members.get(&member_id.to_string()) {
                return roles.contains(member.breach_encapsulation_of_role());
            }
        }
        false
    }

    /// メンバーを追加する
    pub fn add_member(&mut self, member: Member) {
        self.members.insert(
            member.breach_encapsulation_of_id().to_string(),
            member.clone(),
        );
        self.members_ids_by_user_id.insert(
            member.breach_encapsulation_of_user_id().to_string(),
            member.breach_encapsulation_of_id().clone(),
        );
    }

    /// 指定したメンバー ID のメンバーを取得する
    pub fn find_by_id(&self, member_id: &MemberId) -> Option<&Member> {
        self.members.get(&member_id.to_string())
    }

    /// 指定したユーザ ID のメンバーを取得する
    pub fn find_by_user_id(&self, user_id: &UserId) -> Option<&Member> {
        if let Some(member_id) = self.members_ids_by_user_id.get(&user_id.to_string()) {
            self.find_by_id(member_id)
        } else {
            None
        }
    }

    /// 指定したメンバー ID のメンバーを削除する
    pub fn remove_member(&mut self, member_id: &MemberId) {
        if let Some(member) = self.members.remove(&member_id.to_string()) {
            self.members_ids_by_user_id
                .remove(&member.breach_encapsulation_of_user_id().to_string());
        }
    }

    /// 指定したユーザ ID のメンバーを削除する
    pub fn remove_member_by_user_id(&mut self, user_id: &UserId) {
        if let Some(member_id) = self.members_ids_by_user_id.remove(&user_id.to_string()) {
            self.members.remove(&member_id.to_string());
        }
    }

    /// メンバーの一覧を取得する
    pub fn to_vec(&self) -> Vec<&Member> {
        self.members.values().collect()
    }
}

impl Default for Members {
    fn default() -> Self {
        Self::new()
    }
}
