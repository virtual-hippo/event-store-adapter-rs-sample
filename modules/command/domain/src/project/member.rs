use serde::{Deserialize, Serialize};

use crate::project::member_id::MemberId;
use crate::project::member_role::MemberRole;
use crate::user::user_id::UserId;

/// メンバー
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Member {
    id: MemberId,
    user_id: UserId,
    role: MemberRole,
}

/// データのカプセル化を意図的に破るための特別なアクセサメソッド群
impl Member {
    pub fn breach_encapsulation_of_id(&self) -> &MemberId {
        &self.id
    }

    pub fn breach_encapsulation_of_user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn breach_encapsulation_of_role(&self) -> &MemberRole {
        &self.role
    }
}

impl Member {
    pub fn new(id: MemberId, user_id: UserId, role: MemberRole) -> Self {
        Self { id, user_id, role }
    }
}

impl PartialOrd for Member {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}
