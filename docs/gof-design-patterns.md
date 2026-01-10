# RustとDDDで学ぶGoFデザインパターン

yaruタスク管理アプリケーションの実装から理解する23のデザインパターン

---

## はじめに

こんにちは！この記事では、Rust製のタスク管理CLIアプリ「yaru」の実装を通じて、**Gang of Four (GoF) の23のデザインパターン**がどのように活用されているかを解説します。

**Gang of Four (GoF)** とは、エーリヒ・ガンマ、リチャード・ヘルム、ラルフ・ジョンソン、ジョン・ブリシディースの4人の著者によって書かれた名著『オブジェクト指向における再利用のためのデザインパターン』に登場する23のデザインパターンのことです。

yaruプロジェクトは、ドメイン駆動設計（DDD）とクリーンアーキテクチャを採用しており、適切なGoFパターンを活用することで、保守性と拡張性の高い設計を実現しています。

**この記事でわかること：**
- GoFの23パターンの概要
- yaruプロジェクトで使用されている6つのGoFパターン
- DDDで使われるその他の重要なパターン
- パターン同士の相互作用
- パターンを適用すべき場面

---

## 1. GoFの23パターンとは？

GoFのデザインパターンは、**生成**、**構造**、**振る舞い**の3つのカテゴリに分類されます。

### 生成に関するパターン（Creational Patterns）
オブジェクトの生成に関するパターン：
- **Factory Method** ⭐ (yaruで使用)
- Abstract Factory
- **Builder**
- Prototype
- Singleton

### 構造に関するパターン（Structural Patterns）
クラスやオブジェクトの構造に関するパターン：
- **Adapter** ⭐ (yaruで使用)
- Bridge
- Composite
- Decorator
- **Facade** ⭐ (yaruで使用)
- Flyweight
- Proxy

### 振る舞いに関するパターン（Behavioral Patterns）
オブジェクト間の責任分担や協調動作に関するパターン：
- Chain of Responsibility
- Command
- Interpreter
- Iterator
- Mediator
- Memento
- **Observer** ⭐ (yaruで使用)
- State
- **Strategy** ⭐ (yaruで使用)
- **Template Method** ⭐ (yaruで使用)
- Visitor

---

## 2. yaruで使用されているGoFパターン

yaruプロジェクトでは、**23パターンのうち6つ**を明確に実装しています。それぞれを詳しく見ていきましょう。

### 2.1 Strategy パターン 🎯

**分類**: 振る舞いに関するパターン

**概要**: アルゴリズムをカプセル化し、実行時に動的に切り替え可能にするパターンです。

#### 使用箇所
- `src/domain/task/specification.rs`

#### 実装内容

yaruでは、タスクの検索条件を動的に組み合わせるために、Strategyパターンを活用しています。

```rust
/// TaskSpecification - タスクのフィルタリング条件を表すStrategy
pub trait TaskSpecification: Send + Sync {
    /// タスクが条件を満たすかを判定
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool;

    /// AND条件で別の条件と組み合わせる
    fn and(self: Box<Self>, other: Box<dyn TaskSpecification>) -> Box<dyn TaskSpecification>;

    /// OR条件で別の条件と組み合わせる
    fn or(self: Box<Self>, other: Box<dyn TaskSpecification>) -> Box<dyn TaskSpecification>;
}
```

**具体的な戦略（Strategy）クラス**:

```rust
/// ステータスでフィルタリング
pub struct TaskByStatus {
    status: Status,
}

impl TaskSpecification for TaskByStatus {
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool {
        task.status() == &self.status
    }
}

/// 優先度でフィルタリング
pub struct TaskByPriority {
    priority: Priority,
}

impl TaskSpecification for TaskByPriority {
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool {
        task.priority() == &self.priority
    }
}

/// 期限切れでフィルタリング
pub struct TaskOverdue;

impl TaskSpecification for TaskOverdue {
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool {
        task.is_overdue()
    }
}
```

**使用例**:

```rust
// 「優先度が高い」AND「期限切れ」のタスクを検索
let spec = Box::new(TaskByPriority::new(Priority::High))
    .and(Box::new(TaskOverdue));

let filtered_tasks: Vec<_> = tasks
    .into_iter()
    .filter(|task| spec.is_satisfied_by(task))
    .collect();
```

**メリット**:
- ✅ 検索条件を実行時に動的に組み合わせられる
- ✅ 新しい検索条件を追加しても既存コードを変更しない（Open/Closed原則）
- ✅ テストが容易（各条件を独立してテスト可能）

---

### 2.2 Factory Method パターン 🏭

**分類**: 生成に関するパターン

**概要**: オブジェクトの生成をサブクラスに委ねるパターンです。

#### 使用箇所
- `src/domain/task/aggregate.rs`
- `src/domain/tag/aggregate.rs`

#### 実装内容

yaruでは、集約ルート（Aggregate Root）の生成にFactory Methodパターンを使用しています。

```rust
impl TaskAggregate {
    /// 新規作成用のファクトリメソッド
    ///
    /// ビジネスロジックに基づいてTaskAggregateを生成します。
    /// 作成日時、更新日時は自動で設定されます。
    pub fn new(
        title: TaskTitle,
        description: TaskDescription,
        status: Status,
        priority: Priority,
        tags: Vec<TagId>,
        due_date: Option<DueDate>,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: TaskId::new(0).unwrap(), // 永続化時にIDが割り当てられる
            title,
            description,
            status,
            priority,
            tags,
            created_at: now,
            updated_at: now,
            due_date,
            completed_at: None,
            domain_events: Vec::new(),
        }
    }

    /// リポジトリからの再構築用ファクトリメソッド
    ///
    /// データベースから取得したデータをもとに、TaskAggregateを再構築します。
    /// ドメインイベントは空の状態で作成されます。
    pub fn reconstruct(params: TaskReconstructParams) -> Self {
        Self {
            id: params.id,
            title: params.title,
            description: params.description,
            status: params.status,
            priority: params.priority,
            tags: params.tags,
            created_at: params.created_at,
            updated_at: params.updated_at,
            due_date: params.due_date,
            completed_at: params.completed_at,
            domain_events: Vec::new(), // 再構築時はイベントは空
        }
    }
}
```

**2つのファクトリメソッドの使い分け**:

1. **`new()`**: 新規作成時
   - ビジネスロジックに従った初期化
   - `created_at`, `updated_at`を現在時刻に設定
   - IDは仮の値（永続化時に採番）

2. **`reconstruct()`**: データベースからの読み込み時
   - 保存されていたデータをそのまま復元
   - ドメインイベントは空（過去のイベントは復元しない）

**メリット**:
- ✅ オブジェクト生成のロジックを一箇所に集約
- ✅ 不正な状態のオブジェクトが生成されることを防ぐ
- ✅ 生成ロジックの変更が容易

---

### 2.3 Observer パターン 👀

**分類**: 振る舞いに関するパターン

**概要**: オブジェクトの状態変化を他のオブジェクトに通知するパターンです。

#### 使用箇所
- `src/domain/task/events.rs`
- `src/domain/task/aggregate.rs`

#### 実装内容

yaruでは、ドメインイベントを使ってObserverパターンを実装しています。

```rust
/// DomainEvent - すべてのドメインイベントが実装すべきトレイト
pub trait DomainEvent: Debug + Send + Sync {
    /// イベントが発生した日時
    fn occurred_at(&self) -> DateTime<Utc>;

    /// イベントの名前
    fn event_name(&self) -> &str;

    /// ダウンキャスト用
    fn as_any(&self) -> &dyn std::any::Any;
}
```

**具体的なドメインイベント**:

```rust
/// タスク完了イベント
#[derive(Debug, Clone)]
pub struct TaskCompleted {
    task_id: TaskId,
    occurred_at: DateTime<Utc>,
}

impl TaskCompleted {
    pub fn new(task_id: TaskId, occurred_at: DateTime<Utc>) -> Self {
        Self {
            task_id,
            occurred_at,
        }
    }

    pub fn task_id(&self) -> &TaskId {
        &self.task_id
    }
}

impl DomainEvent for TaskCompleted {
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_name(&self) -> &str {
        "TaskCompleted"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
```

**イベントの発行（Subject側）**:

```rust
impl TaskAggregate {
    /// タスクを完了する
    pub fn complete(&mut self) -> Result<()> {
        if self.status != Status::Completed {
            self.status = Status::Completed;
            let now = Utc::now();
            self.completed_at = Some(now);
            self.updated_at = now;

            // イベント発行
            let event = TaskCompleted::new(self.id, now);
            self.domain_events.push(Box::new(event));
        }
        Ok(())
    }

    /// タグを追加する
    pub fn add_tag(&mut self, tag_id: TagId) -> Result<()> {
        if !self.tags.contains(&tag_id) {
            self.tags.push(tag_id);
            self.updated_at = Utc::now();

            // イベント発行
            let event = TaskTagAdded::new(self.id, tag_id, Utc::now());
            self.domain_events.push(Box::new(event));
        }
        Ok(())
    }

    /// ドメインイベントを取得してクリア
    pub fn take_domain_events(&mut self) -> Vec<Box<dyn DomainEvent>> {
        std::mem::take(&mut self.domain_events)
    }
}
```

**メリット**:
- ✅ ドメインロジックの変更を他のコンポーネントに通知できる
- ✅ 疎結合な設計（イベント発行側とイベント処理側が独立）
- ✅ 将来的な拡張が容易（新しいイベントハンドラを追加できる）

**現在のyaruでの活用**:
- タスク完了時の通知
- タイトル変更の記録
- タグの追加・削除の追跡

**将来の拡張例**:
- メール通知の送信
- 統計情報の更新
- 外部システムとの連携

---

### 2.4 Template Method パターン 📋

**分類**: 振る舞いに関するパターン

**概要**: アルゴリズムの骨格を定義し、具体的なステップをサブクラスに委ねるパターンです。

#### 使用箇所
- `src/domain/task/specification.rs`

#### 実装内容

yaruでは、Specificationの組み合わせロジックにTemplate Methodパターンを使用しています。

```rust
pub trait TaskSpecification: Send + Sync {
    /// 具体的な条件判定（サブクラスで実装）
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool;

    /// AND条件の組み合わせ（テンプレートメソッド）
    ///
    /// アルゴリズムの骨格を定義：
    /// 1. 左側の条件をチェック
    /// 2. 右側の条件をチェック
    /// 3. 両方trueならtrue
    fn and(self: Box<Self>, other: Box<dyn TaskSpecification>) -> Box<dyn TaskSpecification> {
        Box::new(AndSpecification {
            left: self,
            right: other,
        })
    }

    /// OR条件の組み合わせ（テンプレートメソッド）
    fn or(self: Box<Self>, other: Box<dyn TaskSpecification>) -> Box<dyn TaskSpecification> {
        Box::new(OrSpecification {
            left: self,
            right: other,
        })
    }
}
```

**AND条件の実装**:

```rust
/// AND条件を表すSpecification
pub struct AndSpecification {
    left: Box<dyn TaskSpecification>,
    right: Box<dyn TaskSpecification>,
}

impl TaskSpecification for AndSpecification {
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool {
        // テンプレートメソッドで定義された骨格に従って実行
        self.left.is_satisfied_by(task) && self.right.is_satisfied_by(task)
    }
}
```

**OR条件の実装**:

```rust
/// OR条件を表すSpecification
pub struct OrSpecification {
    left: Box<dyn TaskSpecification>,
    right: Box<dyn TaskSpecification>,
}

impl TaskSpecification for OrSpecification {
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool {
        self.left.is_satisfied_by(task) || self.right.is_satisfied_by(task)
    }
}
```

**使用例**:

```rust
// (優先度が高い AND ステータスが進行中) OR 期限切れ
let spec = Box::new(TaskByPriority::new(Priority::High))
    .and(Box::new(TaskByStatus::new(Status::InProgress)))
    .or(Box::new(TaskOverdue));
```

**メリット**:
- ✅ 論理演算のアルゴリズム（AND, OR）の骨格が明確
- ✅ 具体的な条件判定は各Specificationに委ねられる
- ✅ コードの重複を避けられる

---

### 2.5 Facade パターン 🏢

**分類**: 構造に関するパターン

**概要**: 複雑なサブシステムに対して、シンプルなインターフェースを提供するパターンです。

#### 使用箇所
- `src/application/use_cases/task/add_task.rs`
- `src/application/use_cases/task/update_task.rs`
- `src/application/use_cases/task/complete_task.rs`
- その他すべてのユースケース

#### 実装内容

yaruでは、ユースケースクラスがFacadeパターンの役割を果たしています。

```rust
/// AddTaskUseCase - タスク追加のユースケース
///
/// 複雑なタスク追加処理を単一の`execute()`メソッドで提供します。
pub struct AddTaskUseCase {
    task_repository: Arc<dyn TaskRepository>,
    tag_repository: Arc<dyn TagRepository>,
}

impl AddTaskUseCase {
    pub fn new(
        task_repository: Arc<dyn TaskRepository>,
        tag_repository: Arc<dyn TagRepository>,
    ) -> Self {
        Self {
            task_repository,
            tag_repository,
        }
    }

    /// タスクを追加する（Facadeメソッド）
    ///
    /// 内部で以下の複雑な処理を実行：
    /// 1. DTOのバリデーション
    /// 2. Value Objectの生成
    /// 3. タグの存在確認
    /// 4. TaskAggregateの生成
    /// 5. リポジトリへの保存
    /// 6. DTOへの変換
    pub async fn execute(&self, dto: CreateTaskDTO) -> Result<TaskDTO> {
        // 1. タイトルのバリデーション
        let title = TaskTitle::new(dto.title)?;

        // 2. 説明のバリデーション
        let description = if let Some(desc) = dto.description {
            TaskDescription::new(desc)?
        } else {
            TaskDescription::new("")?
        };

        // 3. ステータスの変換
        let status = if let Some(status_str) = dto.status {
            Status::from_str(&status_str)
                .or_else(|_| Status::from_filter_value(&status_str))?
        } else {
            Status::Pending
        };

        // 4. 優先度の変換
        let priority = if let Some(priority_str) = dto.priority {
            parse_priority(&priority_str)?
        } else {
            Priority::Medium
        };

        // 5. タグの存在確認
        for tag_id in &dto.tags {
            let tag_id_vo = TagId::new(*tag_id)?;
            if self.tag_repository.find_by_id(&tag_id_vo).await?.is_none() {
                bail!("タグID {}は存在しません", tag_id);
            }
        }

        // 6. タグIDのValue Objectに変換
        let tag_ids: Result<Vec<_>> = dto
            .tags
            .iter()
            .map(|id| TagId::new(*id))
            .collect();
        let tag_ids = tag_ids?;

        // 7. 期限日の変換
        let due_date = dto.due_date.map(DueDate::new).transpose()?;

        // 8. TaskAggregateを作成
        let task = TaskAggregate::new(
            title,
            description,
            status,
            priority,
            tag_ids,
            due_date,
        );

        // 9. リポジトリに保存
        let saved_task = self.task_repository.save(task).await?;

        // 10. DTOに変換して返す
        Ok(TaskDTO::from(saved_task))
    }
}
```

**クライアント側（CLI）からの使用**:

```rust
// 複雑な内部処理は隠蔽され、シンプルなインターフェースで使用できる
let use_case = AddTaskUseCase::new(task_repo, tag_repo);
let created_task = use_case.execute(dto).await?;

println!("タスクを追加しました: [{}] {}", created_task.id, created_task.title);
```

**メリット**:
- ✅ 複雑な処理をシンプルなインターフェース（`execute()`）で提供
- ✅ クライアントコードが簡潔になる
- ✅ 内部の実装詳細を隠蔽できる
- ✅ 変更の影響範囲を限定できる

**yaruでのFacadeの役割**:
1. バリデーション
2. ドメインオブジェクトの生成
3. ビジネスルールの適用
4. リポジトリ操作
5. DTOへの変換

これらすべてを`execute()`メソッド一つで提供しています。

---

### 2.6 Adapter パターン 🔌

**分類**: 構造に関するパターン

**概要**: 互換性のないインターフェースを持つクラス同士を協調動作させるパターンです。

#### 使用箇所
- `src/interface/persistence/sea_orm/mapper.rs`

#### 実装内容

yaruでは、SeaORM（ORM）のデータモデルとドメインモデルの変換にAdapterパターンを使用しています。

```rust
/// TaskMapper - SeaORMモデルとドメインモデルのアダプター
pub struct TaskMapper;

impl TaskMapper {
    /// SeaORM Model → TaskAggregate（Adapter）
    ///
    /// データベースから取得したSeaORMのModelを、ドメイン層のTaskAggregateに変換します。
    pub fn to_domain(task_model: tasks::Model, tag_ids: Vec<i32>) -> Result<TaskAggregate> {
        // IDの変換
        let id = TaskId::new(task_model.id)?;

        // タイトルの変換
        let title = TaskTitle::new(task_model.title)?;

        // 説明の変換
        let description = TaskDescription::new(
            task_model.description.unwrap_or_default()
        )?;

        // ステータスの変換
        let status = Status::from_str(&task_model.status)?;

        // 優先度の変換
        let priority = Priority::from_str(&task_model.priority)?;

        // タグIDsの変換
        let tag_ids: Result<Vec<_>> = tag_ids
            .into_iter()
            .map(TagId::new)
            .collect();
        let tag_ids = tag_ids?;

        // 期限日の変換
        let due_date = task_model.due_date.map(DueDate::new).transpose()?;

        // TaskAggregateの再構築
        let params = TaskReconstructParams {
            id,
            title,
            description,
            status,
            priority,
            tags: tag_ids,
            created_at: task_model.created_at,
            updated_at: task_model.updated_at,
            due_date,
            completed_at: task_model.completed_at,
        };

        Ok(TaskAggregate::reconstruct(params))
    }

    /// TaskAggregate → SeaORM ActiveModel（Adapter）
    ///
    /// ドメイン層のTaskAggregateを、データベースに保存するためのActiveModelに変換します。
    pub fn to_active_model_for_insert(aggregate: &TaskAggregate) -> tasks::ActiveModel {
        tasks::ActiveModel {
            id: NotSet, // 自動採番
            title: Set(aggregate.title().value().to_string()),
            description: Set(Some(aggregate.description().value().to_string())),
            status: Set(aggregate.status().to_string()),
            priority: Set(aggregate.priority().to_string()),
            created_at: Set(aggregate.created_at().clone()),
            updated_at: Set(aggregate.updated_at().clone()),
            due_date: Set(aggregate.due_date().as_ref().map(|d| d.value())),
            completed_at: Set(aggregate.completed_at().clone()),
        }
    }

    /// TaskAggregate → SeaORM ActiveModel（更新用）
    pub fn to_active_model_for_update(aggregate: &TaskAggregate) -> tasks::ActiveModel {
        tasks::ActiveModel {
            id: Unchanged(aggregate.id().value()),
            title: Set(aggregate.title().value().to_string()),
            description: Set(Some(aggregate.description().value().to_string())),
            status: Set(aggregate.status().to_string()),
            priority: Set(aggregate.priority().to_string()),
            created_at: Unchanged(aggregate.created_at().clone()),
            updated_at: Set(aggregate.updated_at().clone()),
            due_date: Set(aggregate.due_date().as_ref().map(|d| d.value())),
            completed_at: Set(aggregate.completed_at().clone()),
        }
    }
}
```

**インピーダンスミスマッチの解決**:

| ドメインモデル | SeaORMモデル | 変換の役割 |
|-------------|------------|----------|
| `TaskTitle(String)` | `String` | Value Objectをプリミティブ型に変換 |
| `Status::InProgress` | `"in_progress"` | Enumを文字列に変換 |
| `TaskId(i32)` | `i32` | Value ObjectをIDに変換 |
| `Option<DueDate>` | `Option<NaiveDate>` | Value Objectを日付型に変換 |

**メリット**:
- ✅ ドメインモデルとORMモデルの違いを吸収
- ✅ ドメイン層がORMに依存しない
- ✅ ORMを変更してもドメイン層は影響を受けない
- ✅ 変換ロジックが一箇所に集約される

**Adapterパターンの恩恵**:
- SeaORMからDieselやSQLxに変更する場合でも、Mapperを書き換えるだけでドメイン層は無変更
- テスト時にInMemoryRepositoryを使う場合も、同じドメインモデルを使用できる

---

## 3. DDDで使われるその他のパターン

GoFの23パターンには含まれませんが、DDDと組み合わせてよく使われるパターンも、yaruで活用されています。

### 3.1 Repository パターン 📚

**概要**: データアクセスを抽象化し、ドメインモデルをコレクションのように扱えるようにするパターンです。

#### 使用箇所
- `src/domain/task/repository.rs` (インターフェース定義)
- `src/domain/tag/repository.rs` (インターフェース定義)
- `src/interface/persistence/sea_orm/task_repository.rs` (SQLite実装)
- `src/interface/persistence/in_memory/task_repository.rs` (メモリ実装)

#### 実装内容

```rust
/// TaskRepository trait - リポジトリのインターフェース
///
/// ドメイン層でインターフェースを定義し、
/// インターフェース層で具体的な実装を提供します（依存性逆転の原則）。
#[async_trait::async_trait]
pub trait TaskRepository: Send + Sync {
    /// IDでタスクを検索
    async fn find_by_id(&self, id: &TaskId) -> Result<Option<TaskAggregate>>;

    /// 全タスクを取得
    async fn find_all(&self) -> Result<Vec<TaskAggregate>>;

    /// 新しいタスクを保存
    async fn save(&self, task: TaskAggregate) -> Result<TaskAggregate>;

    /// 既存のタスクを更新
    async fn update(&self, task: TaskAggregate) -> Result<TaskAggregate>;

    /// IDでタスクを削除
    async fn delete(&self, id: &TaskId) -> Result<bool>;
}
```

**SQLite実装**:

```rust
pub struct SeaOrmTaskRepository {
    db: DatabaseConnection,
}

#[async_trait]
impl TaskRepository for SeaOrmTaskRepository {
    async fn find_by_id(&self, id: &TaskId) -> Result<Option<TaskAggregate>> {
        let task_model = Tasks::find_by_id(id.value()).one(&self.db).await?;

        match task_model {
            Some(model) => {
                let tag_ids = self.get_tag_ids(model.id).await?;
                let aggregate = TaskMapper::to_domain(model, tag_ids)?;
                Ok(Some(aggregate))
            }
            None => Ok(None),
        }
    }

    // ... 他のメソッド実装
}
```

**テスト用メモリ実装**:

```rust
pub struct InMemoryTaskRepository {
    tasks: Arc<RwLock<HashMap<i32, TaskAggregate>>>,
    next_id: Arc<AtomicI32>,
}

#[async_trait]
impl TaskRepository for InMemoryTaskRepository {
    async fn find_by_id(&self, id: &TaskId) -> Result<Option<TaskAggregate>> {
        let tasks = self.tasks.read().await;
        Ok(tasks.get(&id.value()).cloned())
    }

    // ... 他のメソッド実装
}
```

**メリット**:
- ✅ データアクセスロジックをドメインロジックから分離
- ✅ 依存性逆転の原則（DIP）を実現
- ✅ テスト用の実装を簡単に差し替えられる
- ✅ ドメインモデルをコレクションのように扱える

---

### 3.2 Value Object パターン 💎

**概要**: ドメインの概念を表す、不変で交換可能なオブジェクトです。

#### 使用箇所
- `src/domain/task/value_objects/task_title.rs`
- `src/domain/task/value_objects/task_description.rs`
- `src/domain/task/value_objects/priority.rs`
- `src/domain/task/value_objects/status.rs`
- `src/domain/task/value_objects/task_id.rs`
- `src/domain/task/value_objects/due_date.rs`
- その他多数

#### 実装内容

```rust
/// TaskTitle - タスクのタイトルを表すValue Object
///
/// タイトルは1文字以上100文字以内である必要があります。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskTitle(String);

impl TaskTitle {
    /// 新しいTaskTitleを作成
    ///
    /// バリデーションを実施し、不正な値の場合はエラーを返します。
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        // バリデーション1: 空文字チェック
        if value.trim().is_empty() {
            anyhow::bail!("タイトルは空にできません");
        }

        // バリデーション2: 文字数チェック
        if value.len() > 100 {
            anyhow::bail!("タイトルは100文字以内にしてください");
        }

        Ok(Self(value))
    }

    /// タイトルの値を取得
    pub fn value(&self) -> &str {
        &self.0
    }
}
```

**Newtypeパターンの利点**:

```rust
// ❌ プリミティブ型では間違いに気づきにくい
fn create_task(title: String, description: String) { }

// 引数の順序を間違えてもコンパイルエラーにならない
create_task("説明文".to_string(), "タイトル".to_string());

// ✅ Value Objectなら型が違うのでコンパイルエラー
fn create_task(title: TaskTitle, description: TaskDescription) { }

// 順序を間違えるとコンパイルエラーになる
create_task(
    TaskDescription::new("説明文")?,  // ← 型が違うのでエラー
    TaskTitle::new("タイトル")?
);
```

**メリット**:
- ✅ 型安全性の向上（プリミティブ型の混同を防ぐ）
- ✅ ビジネスルールをコンストラクタで強制
- ✅ 不正な値でオブジェクトを作成できない
- ✅ コードの可読性が向上

---

### 3.3 Data Transfer Object (DTO) パターン 📦

**概要**: 層間のデータ転送に特化したオブジェクトです。

#### 使用箇所
- `src/application/dto/task_dto.rs`
- `src/application/dto/tag_dto.rs`
- `src/application/dto/stats_dto.rs`

#### 実装内容

```rust
/// TaskDTO - タスクのデータ転送オブジェクト
///
/// ドメインモデルとプレゼンテーション層の間でデータを転送するために使用します。
/// シリアライズ/デシリアライズ可能で、外部とのやり取りに適しています。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskDTO {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub tags: Vec<TagInfo>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<NaiveDate>,
    pub completed_at: Option<DateTime<Utc>>,
}
```

**ドメインモデルからDTOへの変換**:

```rust
impl From<TaskAggregate> for TaskDTO {
    fn from(aggregate: TaskAggregate) -> Self {
        // タグ情報の解決
        let tags = aggregate
            .tags()
            .iter()
            .map(|tag_id| TagInfo {
                id: tag_id.value(),
                name: String::new(), // 実際にはリポジトリから取得
            })
            .collect();

        Self {
            id: aggregate.id().value(),
            title: aggregate.title().value().to_string(),
            description: Some(aggregate.description().value().to_string()),
            status: aggregate.status().to_string(),
            priority: aggregate.priority().to_string(),
            tags,
            created_at: *aggregate.created_at(),
            updated_at: *aggregate.updated_at(),
            due_date: aggregate.due_date().as_ref().map(|d| d.value()),
            completed_at: *aggregate.completed_at(),
        }
    }
}
```

**メリット**:
- ✅ ドメインモデルを外部に公開しない
- ✅ プレゼンテーション層に適した形式でデータを提供
- ✅ シリアライズ/デシリアライズが容易
- ✅ ドメインモデルの変更が外部に影響しない

---

### 3.4 Singleton パターン（ステートレスサービス）🔧

**概要**: ステートレスなドメインサービスとして実装されています。

#### 使用箇所
- `src/domain/services/task_statistics_service.rs`

#### 実装内容

```rust
/// TaskStatisticsService - タスクの統計情報を計算するドメインサービス
///
/// ステートレスなサービスなので、インスタンス化は不要です。
pub struct TaskStatisticsService;

impl TaskStatisticsService {
    /// タスクの統計情報を計算
    pub fn calculate_stats(tasks: &[TaskAggregate], today: NaiveDate) -> TaskStats {
        let total = tasks.len();

        let pending = tasks
            .iter()
            .filter(|t| t.status() == &Status::Pending)
            .count();

        let in_progress = tasks
            .iter()
            .filter(|t| t.status() == &Status::InProgress)
            .count();

        let completed = tasks
            .iter()
            .filter(|t| t.status() == &Status::Completed)
            .count();

        let overdue = tasks
            .iter()
            .filter(|t| t.is_overdue())
            .count();

        TaskStats {
            total,
            pending,
            in_progress,
            completed,
            overdue,
        }
    }
}
```

**使用例**:

```rust
// インスタンス化せずに直接呼び出す
let stats = TaskStatisticsService::calculate_stats(&tasks, today);
```

**メリット**:
- ✅ 状態を持たないため、スレッドセーフ
- ✅ インスタンス化のオーバーヘッドがない
- ✅ シンプルで理解しやすい

---

## 4. 使用されていないGoFパターン

yaruプロジェクトでは、23パターンのうち17パターンは明示的に使用されていません。

### 生成パターン

- ❌ **Abstract Factory**: 関連するオブジェクトのファミリーを生成する必要性がない
- ❌ **Builder**: Update DTOで部分的に類似しているが、厳密なBuilderパターンではない
- ❌ **Prototype**: オブジェクトのクローンで生成する必要性がない
- ❌ **Singleton**: ステートレスサービスで代替（厳密なSingletonではない）

### 構造パターン

- ❌ **Bridge**: 抽象と実装を独立に変更する必要性がない
- ❌ **Composite**: ツリー構造を表現する必要性がない
- ❌ **Decorator**: 動的に機能を追加する必要性がない
- ❌ **Flyweight**: 大量の細かいオブジェクトを共有する必要性がない
- ❌ **Proxy**: 遅延初期化やアクセス制御の必要性がない

### 振る舞いパターン

- ❌ **Chain of Responsibility**: リクエストを複数のハンドラで処理する必要性がない
- ❌ **Command**: 操作をオブジェクトとしてカプセル化する必要性がない（現状）
- ❌ **Interpreter**: 独自の言語を解釈する必要性がない
- ❌ **Iterator**: Rustの標準Iteratorを使用（GoFパターンとしては実装していない）
- ❌ **Mediator**: 複雑なオブジェクト間の通信を管理する必要性がない
- ❌ **Memento**: オブジェクトの状態を保存・復元する必要性がない
- ❌ **State**: 状態によって振る舞いを変える必要性がない（EnumのStatusで対応）
- ❌ **Visitor**: 構造と操作を分離する必要性がない

**重要な考え方**:
- すべてのパターンを使う必要はありません
- **必要な時に必要なパターンだけを使う**のが正しいアプローチ
- 過剰な設計（Over-engineering）は避けるべき

---

## 5. パターンの相互作用

yaruプロジェクトでは、複数のパターンが組み合わさって、堅牢なアーキテクチャを実現しています。

### 5.1 タスク追加の処理フロー

実際のタスク追加処理を通じて、パターンの連携を見てみましょう。

```
1. CLI層（Interface層）
   ↓
2. Facade: AddTaskUseCase.execute()
   ↓
3. Value Object: TaskTitle::new(), TaskDescription::new()
   ↓
4. Factory Method: TaskAggregate::new()
   ↓
5. Repository: task_repository.save()
   ↓
6. Adapter: TaskMapper::to_active_model_for_insert()
   ↓
7. データベース（SQLite）
```

**各ステップで使われるパターン**:

| ステップ | パターン | 役割 |
|---------|---------|-----|
| 2 | Facade | 複雑な処理をシンプルなインターフェースで提供 |
| 3 | Value Object | ビジネスルールを強制 |
| 4 | Factory Method | 一貫した方法でオブジェクトを生成 |
| 5 | Repository | データアクセスを抽象化 |
| 6 | Adapter | ドメインモデルとORMモデルを変換 |

### 5.2 タスク検索の処理フロー

```
1. CLI層（Interface層）
   ↓
2. Facade: SearchTasksUseCase.execute()
   ↓
3. Strategy: TaskSpecificationの組み合わせ
   ├─ TaskByStatus
   ├─ TaskByPriority
   └─ TaskOverdue
   ↓
4. Template Method: and(), or()
   ↓
5. Repository: task_repository.find_all()
   ↓
6. フィルタリング実行
   ↓
7. DTO: TaskDTO変換
```

**パターンの連携**:

- **Strategy** + **Template Method**: 検索条件を柔軟に組み合わせ
- **Repository**: データ取得を抽象化
- **DTO**: 結果を外部に転送

### 5.3 タスク完了の処理フロー

```
1. CLI層（Interface層）
   ↓
2. Facade: CompleteTaskUseCase.execute()
   ↓
3. Repository: task_repository.find_by_id()
   ↓
4. Adapter: TaskMapper::to_domain()
   ↓
5. ドメインロジック: TaskAggregate.complete()
   ↓
6. Observer: TaskCompletedイベント発行
   ↓
7. Repository: task_repository.update()
   ↓
8. Adapter: TaskMapper::to_active_model_for_update()
```

**イベント駆動アーキテクチャ**:

- **Observer**: タスク完了をイベントとして通知
- **Factory Method**: イベントオブジェクトの生成
- 将来の拡張性（通知送信、統計更新など）を確保

---

## 6. まとめ

### yaruプロジェクトで使用されているパターン一覧

#### GoFの23パターン（6つ使用）

| パターン | 分類 | 使用箇所 | 主な役割 |
|---------|------|---------|---------|
| ⭐ Strategy | 振る舞い | `src/domain/task/specification.rs` | 検索条件の動的な組み合わせ |
| ⭐ Factory Method | 生成 | `src/domain/task/aggregate.rs` | 集約ルートの生成 |
| ⭐ Observer | 振る舞い | `src/domain/task/events.rs` | ドメインイベントの発行 |
| ⭐ Template Method | 振る舞い | `src/domain/task/specification.rs` | 論理演算の骨格定義 |
| ⭐ Facade | 構造 | `src/application/use_cases/` | 複雑な処理の隠蔽 |
| ⭐ Adapter | 構造 | `src/interface/persistence/sea_orm/mapper.rs` | ORMとドメインの変換 |

#### DDDパターン（4つ使用）

| パターン | 使用箇所 | 主な役割 |
|---------|---------|---------|
| Repository | `src/domain/*/repository.rs` | データアクセスの抽象化 |
| Value Object | `src/domain/*/value_objects/` | 型安全性とビジネスルール |
| DTO | `src/application/dto/` | 層間のデータ転送 |
| Singleton的サービス | `src/domain/services/` | ステートレスなドメインサービス |

### パターンを使うメリット

✅ **保守性の向上**
- 各パターンが明確な責務を持つ
- 変更の影響範囲が限定される

✅ **拡張性の確保**
- 新しい機能を追加しやすい
- 既存コードを変更せずに拡張可能（Open/Closed原則）

✅ **テスト容易性**
- 各層を独立してテスト可能
- モックやスタブを簡単に作成できる

✅ **コミュニケーションの効率化**
- チームメンバー間で共通の言語を使える
- 設計意図が明確になる

### パターンを適用すべき場面

👍 **適用すべきケース**:
- ビジネスロジックが複雑なアプリケーション
- 長期運用が予定されているプロジェクト
- チーム開発のプロジェクト
- 技術スタックの変更可能性がある場合

👎 **適用を避けるべきケース**:
- シンプルなCRUDアプリケーション
- 使い捨てのプロトタイプ
- 個人の小規模プロジェクト（学習目的は除く）
- 納期が非常に短いプロジェクト

### 設計の原則

yaruプロジェクトは、以下のSOLID原則に従って設計されています：

1. **Single Responsibility Principle（単一責任の原則）**
   - 各クラス・モジュールは一つの責務のみを持つ

2. **Open/Closed Principle（開放/閉鎖の原則）**
   - 拡張に対して開いており、修正に対して閉じている

3. **Liskov Substitution Principle（リスコフの置換原則）**
   - 派生型は基底型と置き換え可能

4. **Interface Segregation Principle（インターフェース分離の原則）**
   - クライアントは使用しないインターフェースに依存しない

5. **Dependency Inversion Principle（依存性逆転の原則）**
   - 抽象に依存し、具体に依存しない

---

## おわりに

yaruプロジェクトを通じて、GoFの23パターンのうち6つのパターンと、DDDでよく使われる4つのパターンを紹介しました。

重要なのは、**「パターンを使うこと」が目的ではなく、「問題を解決すること」が目的**だということです。パターンは、よくある問題に対する実証済みの解決策を提供してくれますが、すべての問題に適用すべきではありません。

yaruプロジェクトのように、DDDとクリーンアーキテクチャを組み合わせ、適切なパターンを選択することで、保守性と拡張性の高いソフトウェアを構築できます。

**参考リンク**:
- [Gang of Four - Design Patterns](https://en.wikipedia.org/wiki/Design_Patterns)
- [Eric Evans - Domain-Driven Design](https://www.domainlanguage.com/ddd/)
- [Robert C. Martin - Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)

Happy Coding! 🦀
