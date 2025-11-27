use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// 銃砲ID値オブジェクト
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FirearmId(Uuid);

impl FirearmId {
    /// 新しいFirearmIdを生成
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// UUIDから生成
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// UUID文字列から生成
    pub fn from_str(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }

    /// UUIDとして取得
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// UUIDとして取得（所有権）
    pub fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for FirearmId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for FirearmId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for FirearmId {
    fn from(uuid: Uuid) -> Self {
        Self::from_uuid(uuid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_firearm_id() {
        let id1 = FirearmId::new();
        let id2 = FirearmId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_from_uuid() {
        let uuid = Uuid::new_v4();
        let firearm_id = FirearmId::from_uuid(uuid);
        assert_eq!(firearm_id.as_uuid(), &uuid);
    }

    #[test]
    fn test_from_str() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let firearm_id = FirearmId::from_str(uuid_str).unwrap();
        assert_eq!(firearm_id.to_string(), uuid_str);
    }
}
