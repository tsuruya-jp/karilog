use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// 実包種別ID値オブジェクト
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AmmunitionTypeId(Uuid);

impl AmmunitionTypeId {
    /// 新しいAmmunitionTypeIdを生成
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

impl Default for AmmunitionTypeId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AmmunitionTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for AmmunitionTypeId {
    fn from(uuid: Uuid) -> Self {
        Self::from_uuid(uuid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ammunition_type_id() {
        let id1 = AmmunitionTypeId::new();
        let id2 = AmmunitionTypeId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_from_uuid() {
        let uuid = Uuid::new_v4();
        let ammunition_type_id = AmmunitionTypeId::from_uuid(uuid);
        assert_eq!(ammunition_type_id.as_uuid(), &uuid);
    }

    #[test]
    fn test_from_str() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let ammunition_type_id = AmmunitionTypeId::from_str(uuid_str).unwrap();
        assert_eq!(ammunition_type_id.to_string(), uuid_str);
    }
}
