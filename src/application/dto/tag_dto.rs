use crate::domain::tag::aggregate::TagAggregate;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// タグの読み取り専用表現（DTO）
///
/// Use CaseからPresentation層への出力に使用されます。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TagDTO {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// タグ作成時の入力DTO
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateTagDTO {
    pub name: String,
    pub description: Option<String>,
}

/// タグ更新時の入力DTO
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateTagDTO {
    pub name: Option<String>,
    pub description: Option<String>,
}

// TagAggregateからTagDTOへの変換
impl From<TagAggregate> for TagDTO {
    fn from(tag: TagAggregate) -> Self {
        Self {
            id: tag.id().value(),
            name: tag.name().value().to_string(),
            description: if tag.description().value().is_empty() {
                None
            } else {
                Some(tag.description().value().to_string())
            },
            created_at: *tag.created_at(),
            updated_at: *tag.updated_at(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tag::{
        aggregate::TagAggregate,
        value_objects::{TagDescription, TagName},
    };

    #[test]
    fn test_tag_dto_from_aggregate() {
        let tag = TagAggregate::new(
            TagName::new("重要").unwrap(),
            TagDescription::new("重要なタスク").unwrap(),
        );

        let dto = TagDTO::from(tag);

        assert_eq!(dto.id, 0); // デフォルトID
        assert_eq!(dto.name, "重要");
        assert_eq!(dto.description, Some("重要なタスク".to_string()));
    }

    #[test]
    fn test_tag_dto_from_aggregate_empty_description() {
        let tag = TagAggregate::new(
            TagName::new("タグ").unwrap(),
            TagDescription::new("").unwrap(),
        );

        let dto = TagDTO::from(tag);

        assert_eq!(dto.name, "タグ");
        assert_eq!(dto.description, None);
    }

    #[test]
    fn test_create_tag_dto() {
        let dto = CreateTagDTO {
            name: "新しいタグ".to_string(),
            description: Some("説明".to_string()),
        };

        assert_eq!(dto.name, "新しいタグ");
        assert_eq!(dto.description, Some("説明".to_string()));
    }

    #[test]
    fn test_create_tag_dto_no_description() {
        let dto = CreateTagDTO {
            name: "タグ".to_string(),
            description: None,
        };

        assert_eq!(dto.name, "タグ");
        assert_eq!(dto.description, None);
    }

    #[test]
    fn test_update_tag_dto_default() {
        let dto = UpdateTagDTO::default();

        assert_eq!(dto.name, None);
        assert_eq!(dto.description, None);
    }

    #[test]
    fn test_update_tag_dto_partial() {
        let dto = UpdateTagDTO {
            name: Some("更新名".to_string()),
            ..Default::default()
        };

        assert_eq!(dto.name, Some("更新名".to_string()));
        assert_eq!(dto.description, None);
    }
}
