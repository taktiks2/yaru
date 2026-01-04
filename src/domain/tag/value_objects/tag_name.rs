use anyhow::Result;

/// タグの名前を表すValue Object
///
/// 名前は1文字以上50文字以内の文字列です。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TagName(String);

impl TagName {
    /// 新しいTagNameを作成
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            anyhow::bail!("タグ名は空にできません");
        }
        if value.len() > 50 {
            anyhow::bail!("タグ名は50文字以内にしてください");
        }
        Ok(Self(value))
    }

    /// 名前の値を取得
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_name_valid() {
        let name = TagName::new("重要").unwrap();
        assert_eq!(name.value(), "重要");
    }

    #[test]
    fn test_tag_name_single_char() {
        let name = TagName::new("A").unwrap();
        assert_eq!(name.value(), "A");
    }

    #[test]
    fn test_tag_name_max_length() {
        let long_name = "a".repeat(50);
        let name = TagName::new(&long_name).unwrap();
        assert_eq!(name.value(), long_name);
    }

    #[test]
    fn test_tag_name_empty() {
        let result = TagName::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_tag_name_whitespace_only() {
        let result = TagName::new("   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_tag_name_too_long() {
        let result = TagName::new("a".repeat(51));
        assert!(result.is_err());
    }

    #[test]
    fn test_tag_name_with_whitespace() {
        let name = TagName::new(" 重要 ").unwrap();
        assert_eq!(name.value(), " 重要 ");
    }

    #[test]
    fn test_tag_name_equality() {
        let name1 = TagName::new("重要").unwrap();
        let name2 = TagName::new("重要").unwrap();
        assert_eq!(name1, name2);
    }

    #[test]
    fn test_tag_name_clone() {
        let name1 = TagName::new("重要").unwrap();
        let name2 = name1.clone();
        assert_eq!(name1, name2);
    }

    #[test]
    fn test_tag_name_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let name = TagName::new("重要").unwrap();
        set.insert(name.clone());
        assert!(set.contains(&name));
    }
}
