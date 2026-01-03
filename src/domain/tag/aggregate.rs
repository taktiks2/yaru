use anyhow::Result;
use chrono::{DateTime, Utc};

use super::value_objects::{TagDescription, TagId, TagName};

/// TagAggregate - タグのAggregate Root
///
/// タグのビジネスルールを実装し、不変条件を保護します。
#[derive(Debug, Clone, PartialEq)]
pub struct TagAggregate {
    id: TagId,
    name: TagName,
    description: TagDescription,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TagAggregate {
    /// 新しいタグを作成します（ファクトリメソッド）
    pub fn new(name: TagName, description: TagDescription) -> Self {
        let now = Utc::now();
        Self {
            id: TagId::new(0).unwrap(), // デフォルトは0、リポジトリで新しいIDを割り当てる
            name,
            description,
            created_at: now,
            updated_at: now,
        }
    }

    /// タグの名前を変更します
    pub fn change_name(&mut self, new_name: TagName) -> Result<()> {
        self.name = new_name;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// タグの説明を変更します
    pub fn change_description(&mut self, new_description: TagDescription) -> Result<()> {
        self.description = new_description;
        self.updated_at = Utc::now();
        Ok(())
    }

    // Getters
    pub fn id(&self) -> &TagId {
        &self.id
    }

    pub fn name(&self) -> &TagName {
        &self.name
    }

    pub fn description(&self) -> &TagDescription {
        &self.description
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tag() {
        // Arrange
        let name = TagName::new("重要").unwrap();
        let description = TagDescription::new("重要なタスク").unwrap();

        // Act
        let tag = TagAggregate::new(name.clone(), description.clone());

        // Assert
        assert_eq!(tag.name(), &name);
        assert_eq!(tag.description(), &description);
    }

    #[test]
    fn test_change_name() {
        // Arrange
        let name = TagName::new("元の名前").unwrap();
        let description = TagDescription::new("説明").unwrap();
        let mut tag = TagAggregate::new(name, description);
        let new_name = TagName::new("新しい名前").unwrap();

        // Act
        let result = tag.change_name(new_name.clone());

        // Assert
        assert!(result.is_ok());
        assert_eq!(tag.name(), &new_name);
    }

    #[test]
    fn test_change_description() {
        // Arrange
        let name = TagName::new("タグ").unwrap();
        let description = TagDescription::new("元の説明").unwrap();
        let mut tag = TagAggregate::new(name, description);
        let new_description = TagDescription::new("新しい説明").unwrap();

        // Act
        let result = tag.change_description(new_description.clone());

        // Assert
        assert!(result.is_ok());
        assert_eq!(tag.description(), &new_description);
    }
}
