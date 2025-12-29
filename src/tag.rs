use crate::repository::HasId;
use chrono::Utc;
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
    pub id: u64,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
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
    pub fn new(id: u64, name: &str, description: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        }
    }
}

impl HasId for Tag {
    fn id(&self) -> u64 {
        self.id
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
