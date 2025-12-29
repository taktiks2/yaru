use crate::json::{load_json, save_json};
use anyhow::Result;
use serde::{Serialize, de::DeserializeOwned};
use std::marker::PhantomData;
use std::path::PathBuf;

/// IDを持つエンティティのトレイト
pub trait HasId {
    /// エンティティのIDを取得
    fn id(&self) -> u64;
}

/// ジェネリックなリポジトリトレイト
/// データの永続化方法を抽象化し、異なる実装（JSON、SQLiteなど）を切り替え可能にする
pub trait Repository<T: HasId> {
    /// データリストを読み込む
    fn load(&self) -> Result<Vec<T>>;

    /// データリストを保存する
    fn save(&self, items: &[T]) -> Result<()>;

    /// 次のIDを取得する
    fn find_next_id(&self, items: &[T]) -> u64 {
        items.iter().map(|item| item.id()).max().unwrap_or(0) + 1
    }

    /// データファイルが存在することを確認（必要に応じて初期化）
    fn ensure_data_exists(&self) -> Result<()>;
}

/// JSON形式でデータを保存するジェネリックなリポジトリ実装
pub struct JsonRepository<T> {
    file_path: PathBuf,
    /// 型パラメータTとの関連を示すマーカー
    ///
    /// PhantomData<T>は実行時にメモリを消費せず（ゼロコスト）、
    /// コンパイラに「この構造体は型Tと関連がある」ことを伝える。
    /// これにより：
    /// - JsonRepository<Task> と JsonRepository<Tag> を別の型として扱える
    /// - load()が返すVec<T>の型安全性が保証される
    /// - 型パラメータTの「未使用」エラーを回避できる
    _phantom: PhantomData<T>,
}

// 注: implブロックが2つに分かれているのはRustの仕様によるものです
// 1. 固有実装（inherent implementation）: 構造体固有のメソッド（newなど）
// 2. トレイト実装（trait implementation）: Repositoryトレイトの実装
// これらを1つのimplブロックにまとめることはできません

impl<T> JsonRepository<T> {
    /// 新しいJsonRepositoryインスタンスを作成
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: file_path.into(),
            _phantom: PhantomData,
        }
    }
}

impl<T> Repository<T> for JsonRepository<T>
where
    T: HasId + Serialize + DeserializeOwned,
{
    fn load(&self) -> Result<Vec<T>> {
        load_json(&self.file_path)
    }

    fn save(&self, items: &[T]) -> Result<()> {
        save_json(&self.file_path, items)
    }

    fn ensure_data_exists(&self) -> Result<()> {
        if !self.file_path.exists() {
            save_json(&self.file_path, &Vec::<T>::new())?;
        }
        Ok(())
    }
}
