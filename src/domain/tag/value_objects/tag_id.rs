use anyhow::Result;

/// タグのIDを表すValue Object
///
/// IDは0以上の整数です。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TagId(i32);

impl TagId {
    /// 新しいTagIdを作成
    pub fn new(value: i32) -> Result<Self> {
        if value < 0 {
            anyhow::bail!("タグIDは0以上である必要があります");
        }
        Ok(Self(value))
    }

    /// IDの値を取得
    pub fn value(&self) -> i32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_id_valid() {
        let id = TagId::new(1).unwrap();
        assert_eq!(id.value(), 1);
    }

    #[test]
    fn test_tag_id_zero() {
        let id = TagId::new(0).unwrap();
        assert_eq!(id.value(), 0);
    }

    #[test]
    fn test_tag_id_large() {
        let id = TagId::new(999999).unwrap();
        assert_eq!(id.value(), 999999);
    }

    #[test]
    fn test_tag_id_negative() {
        let result = TagId::new(-1);
        assert!(result.is_err());
    }

    #[test]
    fn test_tag_id_equality() {
        let id1 = TagId::new(1).unwrap();
        let id2 = TagId::new(1).unwrap();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_tag_id_inequality() {
        let id1 = TagId::new(1).unwrap();
        let id2 = TagId::new(2).unwrap();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_tag_id_copy() {
        let id1 = TagId::new(1).unwrap();
        let id2 = id1; // Copy trait
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_tag_id_hash() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let id = TagId::new(1).unwrap();
        map.insert(id, "test");
        assert_eq!(map.get(&id), Some(&"test"));
    }
}
