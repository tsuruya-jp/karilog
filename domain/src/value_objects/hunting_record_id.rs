use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// 出猟記録ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HuntingRecordId(Uuid);

impl HuntingRecordId {
    /// 新しいIDを生成
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// UUIDから生成
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// UUIDとして取得
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }

    /// 文字列から生成
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl Default for HuntingRecordId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for HuntingRecordId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for HuntingRecordId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<HuntingRecordId> for Uuid {
    fn from(id: HuntingRecordId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_id() {
        let id1 = HuntingRecordId::new();
        let id2 = HuntingRecordId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_from_uuid() {
        let uuid = Uuid::new_v4();
        let id = HuntingRecordId::from_uuid(uuid);
        assert_eq!(id.as_uuid(), uuid);
    }

    #[test]
    fn test_display() {
        let uuid = Uuid::new_v4();
        let id = HuntingRecordId::from_uuid(uuid);
        assert_eq!(id.to_string(), uuid.to_string());
    }
}
