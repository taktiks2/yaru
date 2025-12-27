use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

/// JSONファイルを読み込む関数
/// 型は使うときに決定できるようにジェネリックで実装
///
/// # 高階トレイト境界（HRTB: Higher-Rank Trait Bounds）について
///
/// `T: for<'de> Deserialize<'de>` は「任意のライフタイム 'de に対して Deserialize できる型」を意味する。
///
/// ## なぜ必要なのか
///
/// この関数では：
/// 1. 関数内で `content: String` を作成（関数内でのみ存在する一時データ）
/// 2. `&content` を借用してデシリアライズ
/// 3. デシリアライズ結果 `T` を返す（所有データとして）
/// 4. `content` は関数終了時に破棄される
///
/// もし `T: Deserialize<'a>` と書くと、`'a` は関数シグネチャで定義された
/// 特定のライフタイムになり、関数内で作られる `content` のライフタイムを
/// 表現できない（`content` は関数の外から来るものではないため）。
///
/// `for<'de>` を使うことで、「どんなライフタイムのデータからでも
/// デシリアライズして所有データを作れる型」という柔軟性を持たせている。
///
/// ## 所有データ vs 借用データ
///
/// - 所有データ: `String`, `Vec<T>` など、実際のデータを所有している
///   → 関数から返すことができる
/// - 借用データ: `&str`, `&T` など、他のデータへの参照
///   → 元のデータが破棄されると使えなくなるため、関数から返せない
///
/// この関数が返す `T` は所有データでなければならない。
/// `for<'de>` により、一時的な借用からでも所有データを作れることを保証している。
///
/// # `?Sized` が不要な理由
///
/// この関数は `T` を値として返します（`-> Result<T>`）。
/// 値として返す場合、その値をスタックに配置する必要があるため、
/// コンパイル時にサイズが分かっていなければなりません。
///
/// そのため、`T` には暗黙的に `Sized` 制約が必要で、`?Sized` は使えません。
///
/// ```rust
/// // これはできない
/// let x: [i32] = load_json(...);  // ❌ [i32] はサイズ不定
///
/// // これならOK
/// let x: Vec<i32> = load_json(...);  // ✓ Vec<i32> は固定サイズ（24バイト）
/// ```
///
/// 一方、`save_json` は `&T` を受け取るため、参照自体のサイズが固定なので
/// `?Sized` を付けてサイズ不定の型も受け入れられます。
pub fn load_json<T>(path: impl AsRef<Path>) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let path = path.as_ref();
    let content = fs::read_to_string(path)
        .with_context(|| format!("ファイルの読み込みに失敗しました: {}", path.display()))?; // ① String を所有（関数内のみ）
    let data: T = serde_json::from_str(&content).context("JSONの解析に失敗しました")?; // ② &content から T を作る（所有データ）
    Ok(data) // ③ T を返す（content は破棄されるが、T は独立している）
}

/// JSONファイルを書き出す関数
///
/// # `?Sized` について
///
/// Rust では、デフォルトで全てのジェネリック型パラメータに暗黙的に `Sized` 制約が付きます。
/// つまり、コンパイル時にサイズが既知の型のみ受け入れます。
///
/// `?Sized` を付けることで、「Sized でなくてもよい」= サイズが不定の型も受け入れられるようになります。
///
/// ## なぜ安全か
///
/// この関数は `data: &T` という参照を受け取ります。参照自体のサイズは常に固定（ポインタサイズ）なので、
/// `T` 自体のサイズが不定でも問題ありません。
///
/// ## 具体的な利点
///
/// `?Sized` により、以下のような呼び出しが可能になります：
///
/// ```rust
/// // Vec<i32> は構造体自体が固定サイズ（24バイト）
/// let vec = vec![1, 2, 3];
/// save_json("path.json", &vec);  // T = Vec<i32> ✓
///
/// // [i32] はサイズ不定（要素数が実行時に決まる）
/// let slice: &[i32] = &[1, 2, 3];
/// save_json("path.json", slice);  // T = [i32] ✓ (?Sized がないとエラー)
///
/// // str も サイズ不定
/// let s: &str = "hello";
/// save_json("path.json", s);  // T = str ✓ (?Sized がないとエラー)
/// ```
pub fn save_json<T>(path: impl AsRef<Path>, data: &T) -> Result<()>
where
    T: Serialize + ?Sized,
{
    let path = path.as_ref();

    // 親ディレクトリを作成（存在しない場合）
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("ディレクトリの作成に失敗しました: {}", parent.display()))?;
    }

    let json = serde_json::to_string_pretty(data).context("JSONのシリアライズに失敗しました")?;
    fs::write(path, json)
        .with_context(|| format!("ファイルの書き込みに失敗しました: {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::fs;
    use std::path::PathBuf;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestData {
        id: u64,
        name: String,
    }

    fn get_test_file_path(filename: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(filename);
        path
    }

    #[test]
    fn test_save_json_creates_file() {
        let test_file = get_test_file_path("test_save.json");
        let _ = fs::remove_file(&test_file); // クリーンアップ

        let data = vec![
            TestData {
                id: 1,
                name: "テスト1".to_string(),
            },
            TestData {
                id: 2,
                name: "テスト2".to_string(),
            },
        ];

        let result = save_json(&test_file, &data);
        assert!(result.is_ok(), "save_jsonは成功すべき");
        assert!(test_file.exists(), "ファイルが作成されるべき");

        // クリーンアップ
        let _ = fs::remove_file(&test_file);
    }

    #[test]
    fn test_load_json_reads_data() {
        let test_file = get_test_file_path("test_load.json");

        // テストデータを直接書き込む
        let data = vec![
            TestData {
                id: 1,
                name: "テスト1".to_string(),
            },
            TestData {
                id: 2,
                name: "テスト2".to_string(),
            },
        ];
        let json = serde_json::to_string_pretty(&data).expect("JSONシリアライズは成功すべき");
        fs::write(&test_file, json).expect("ファイル書き込みは成功すべき");

        // load_jsonでデータを読み込む
        let result: Result<Vec<TestData>> = load_json(&test_file);
        assert!(result.is_ok(), "load_jsonは成功すべき");

        let loaded_data = result.expect("データの読み込みは成功すべき");
        assert_eq!(
            loaded_data, data,
            "読み込んだデータは元のデータと一致すべき"
        );

        // クリーンアップ
        let _ = fs::remove_file(&test_file);
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let test_file = get_test_file_path("test_roundtrip.json");
        let _ = fs::remove_file(&test_file);

        let original_data = vec![
            TestData {
                id: 100,
                name: "ラウンドトリップ".to_string(),
            },
            TestData {
                id: 200,
                name: "テスト".to_string(),
            },
        ];

        // 保存
        save_json(&test_file, &original_data).expect("保存は成功すべき");

        // 読み込み
        let loaded_data: Vec<TestData> = load_json(&test_file).expect("読み込みは成功すべき");

        assert_eq!(
            loaded_data, original_data,
            "保存と読み込みでデータが一致すべき"
        );

        // クリーンアップ
        let _ = fs::remove_file(&test_file);
    }

    #[test]
    fn test_load_json_with_nonexistent_file() {
        let test_file = get_test_file_path("nonexistent.json");
        let _ = fs::remove_file(&test_file); // 確実に存在しないようにする

        let result: Result<Vec<TestData>> = load_json(&test_file);
        assert!(
            result.is_err(),
            "存在しないファイルの読み込みはエラーになるべき"
        );
    }

    #[test]
    fn test_load_json_with_invalid_json() {
        let test_file = get_test_file_path("invalid.json");

        // 不正なJSONを書き込む
        fs::write(&test_file, "{ invalid json }").expect("ファイル書き込みは成功すべき");

        let result: Result<Vec<TestData>> = load_json(&test_file);
        assert!(result.is_err(), "不正なJSONの読み込みはエラーになるべき");

        // クリーンアップ
        let _ = fs::remove_file(&test_file);
    }
}
