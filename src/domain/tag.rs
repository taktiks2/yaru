use chrono::{DateTime, Utc};
use entity::tags;
use serde::{Deserialize, Serialize};
use std::fmt;

/// タグを表す構造体
///
/// # フィールド
/// - `id`: タグの一意な識別子
/// - `name`: タグの名前
/// - `description`: タグの説明
/// - `created_at`: タグの作成日時
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Tag {
    /// 新しいTagを作成
    ///
    /// # 引数
    /// - `id`: タグのID
    /// - `name`: タグの名前
    /// - `description`: タグの説明
    ///
    /// # 戻り値
    /// 現在時刻（UTC）を`created_at`に設定した新しいTagインスタンス
    pub fn new(id: i32, name: &str, description: &str) -> Self {
        let now = Utc::now();
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            created_at: now,
            updated_at: now,
        }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// tags::ModelからTagドメインモデルへの変換
impl From<tags::Model> for Tag {
    fn from(model: tags::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: model.description,
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
        }
    }
}
