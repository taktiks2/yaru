use crate::repository::{HasId, Repository};
use crate::task::{Priority, Status, Task};
use crate::tag::Tag;
use anyhow::{Context, Result};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use std::marker::PhantomData;

/// SQLite形式でデータを保存するジェネリックなリポジトリ実装
pub struct SqliteRepository<T> {
    db: DatabaseConnection,
    _phantom: PhantomData<T>,
}

impl<T> SqliteRepository<T> {
    /// 新しいSqliteRepositoryインスタンスを作成
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            _phantom: PhantomData,
        }
    }
}

// Task用の特殊化実装
impl Repository<Task> for SqliteRepository<Task> {
    fn load(&self) -> Result<Vec<Task>> {
        // 非同期処理をブロッキング実行
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
            use crate::entity::tasks;

            let task_models = tasks::Entity::find()
                .all(&self.db)
                .await
                .context("タスクの読み込みに失敗しました")?;

            // EntityからドメインモデルTaskへの変換
            let tasks: Result<Vec<Task>> = task_models
                .into_iter()
                .map(|model| Self::entity_to_domain(model, &self.db))
                .collect();

            tasks
            })
        })
    }

    fn save(&self, items: &[Task]) -> Result<()> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
            use crate::entity::{task_tags, tasks};
            use sea_orm::TransactionTrait;

            // トランザクション内で全件削除 & 再挿入
            let txn = self.db.begin().await?;

            // タスクタグ中間テーブルを削除
            task_tags::Entity::delete_many()
                .exec(&txn)
                .await
                .context("タスクタグの削除に失敗しました")?;

            // 既存タスクデータ削除
            tasks::Entity::delete_many()
                .exec(&txn)
                .await
                .context("タスクの削除に失敗しました")?;

            // 新規データ挿入
            for item in items {
                let active_model = Self::domain_to_active_model(item);
                let inserted = active_model
                    .insert(&txn)
                    .await
                    .context("タスクの挿入に失敗しました")?;

                // タグの関連付けを保存
                for tag_id in &item.tags {
                    let task_tag = task_tags::ActiveModel {
                        task_id: Set(inserted.id.into()),
                        tag_id: Set(*tag_id as i64),
                    };
                    task_tag
                        .insert(&txn)
                        .await
                        .context("タスクタグの挿入に失敗しました")?;
                }
            }

            txn.commit().await.context("トランザクションのコミットに失敗しました")?;
            Ok(())
            })
        })
    }

    fn find_next_id(&self, items: &[Task]) -> u64 {
        items.iter().map(|item| item.id()).max().unwrap_or(0) + 1
    }

    fn ensure_data_exists(&self) -> Result<()> {
        // マイグレーションで既にテーブルが作成されているため、
        // 特に処理は不要
        Ok(())
    }
}

impl SqliteRepository<Task> {
    /// Entityモデルからドメインモデルへ変換
    fn entity_to_domain(
        model: crate::entity::tasks::Model,
        db: &DatabaseConnection,
    ) -> Result<Task> {
        // タグIDを取得（同期的に実行）
        let tag_ids = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
            use crate::entity::task_tags;
            use sea_orm::ColumnTrait;
            use sea_orm::QueryFilter;

            let task_tag_models = task_tags::Entity::find()
                .filter(task_tags::Column::TaskId.eq(model.id))
                .all(db)
                .await
                .context("タグIDの読み込みに失敗しました")?;

            Ok::<Vec<u64>, anyhow::Error>(
                task_tag_models
                    .into_iter()
                    .map(|tt| tt.tag_id as u64)
                    .collect(),
            )
            })
        })?;

        Ok(Task {
            id: model.id as u64,
            title: model.title,
            description: model.description,
            status: Status::from_filter_value(&model.status)
                .unwrap_or(Status::Pending),
            priority: match model.priority.as_str() {
                "Low" => Priority::Low,
                "Medium" => Priority::Medium,
                "High" => Priority::High,
                "Critical" => Priority::Critical,
                _ => Priority::Medium,
            },
            tags: tag_ids,
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
        })
    }

    /// ドメインモデルからActiveModelへ変換
    fn domain_to_active_model(task: &Task) -> crate::entity::tasks::ActiveModel {
        use crate::entity::tasks;
        use chrono::DateTime;

        tasks::ActiveModel {
            id: Set(task.id as i32),
            title: Set(task.title.clone()),
            description: Set(task.description.clone()),
            status: Set(format!("{:?}", task.status)),
            priority: Set(format!("{:?}", task.priority)),
            created_at: Set(
                DateTime::parse_from_rfc3339(&task.created_at)
                    .unwrap()
                    .into(),
            ),
            updated_at: Set(
                DateTime::parse_from_rfc3339(&task.updated_at)
                    .unwrap()
                    .into(),
            ),
        }
    }
}

// Tag用の特殊化実装
impl Repository<Tag> for SqliteRepository<Tag> {
    fn load(&self) -> Result<Vec<Tag>> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
            use crate::entity::tags;

            let tag_models = tags::Entity::find()
                .all(&self.db)
                .await
                .context("タグの読み込みに失敗しました")?;

            let tags: Vec<Tag> = tag_models
                .into_iter()
                .map(Self::tag_entity_to_domain)
                .collect();

            Ok(tags)
            })
        })
    }

    fn save(&self, items: &[Tag]) -> Result<()> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
            use crate::entity::tags;
            use sea_orm::TransactionTrait;

            let txn = self.db.begin().await?;

            // 既存タグデータ削除
            tags::Entity::delete_many()
                .exec(&txn)
                .await
                .context("タグの削除に失敗しました")?;

            // 新規データ挿入
            for item in items {
                let active_model = Self::tag_domain_to_active_model(item);
                active_model
                    .insert(&txn)
                    .await
                    .context("タグの挿入に失敗しました")?;
            }

            txn.commit()
                .await
                .context("トランザクションのコミットに失敗しました")?;
            Ok(())
            })
        })
    }

    fn find_next_id(&self, items: &[Tag]) -> u64 {
        items.iter().map(|item| item.id()).max().unwrap_or(0) + 1
    }

    fn ensure_data_exists(&self) -> Result<()> {
        Ok(())
    }
}

impl SqliteRepository<Tag> {
    /// TagのEntityモデルからドメインモデルへ変換
    fn tag_entity_to_domain(model: crate::entity::tags::Model) -> Tag {
        Tag {
            id: model.id as u64,
            name: model.name,
            description: model.description,
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
        }
    }

    /// TagのドメインモデルからActiveModelへ変換
    fn tag_domain_to_active_model(tag: &Tag) -> crate::entity::tags::ActiveModel {
        use crate::entity::tags;
        use chrono::DateTime;

        tags::ActiveModel {
            id: Set(tag.id as i32),
            name: Set(tag.name.clone()),
            description: Set(tag.description.clone()),
            created_at: Set(
                DateTime::parse_from_rfc3339(&tag.created_at)
                    .unwrap()
                    .into(),
            ),
            updated_at: Set(
                DateTime::parse_from_rfc3339(&tag.updated_at)
                    .unwrap()
                    .into(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use migration::MigratorTrait;
    use sea_orm::Database;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_load_empty_tasks() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        migration::Migrator::up(&db, None).await.unwrap();

        let repo = SqliteRepository::<Task>::new(db);
        let tasks = repo.load().unwrap();
        assert_eq!(tasks.len(), 0);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_load_empty_tags() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        migration::Migrator::up(&db, None).await.unwrap();

        let repo = SqliteRepository::<Tag>::new(db);
        let tags = repo.load().unwrap();
        assert_eq!(tags.len(), 0);
    }
}
