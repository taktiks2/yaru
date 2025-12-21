use anyhow::Result;
use serde::{Deserialize, Serialize};

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
pub fn load_json<T>(path: &str) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let content = std::fs::read_to_string(path)?; // ① String を所有（関数内のみ）
    let data: T = serde_json::from_str(&content)?; // ② &content から T を作る（所有データ）
    Ok(data) // ③ T を返す（content は破棄されるが、T は独立している）
}

/// JSONファイルを書き出す関数
pub fn save_json<T>(path: &str, data: &T) -> Result<()>
where
    T: Serialize,
{
    let json = serde_json::to_string_pretty(data)?;
    std::fs::write(path, json)?;
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

        let result = save_json(test_file.to_str().unwrap(), &data);
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
        let json = serde_json::to_string_pretty(&data).unwrap();
        fs::write(&test_file, json).unwrap();

        // load_jsonでデータを読み込む
        let result: Result<Vec<TestData>> = load_json(test_file.to_str().unwrap());
        assert!(result.is_ok(), "load_jsonは成功すべき");

        let loaded_data = result.unwrap();
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
        save_json(test_file.to_str().unwrap(), &original_data).unwrap();

        // 読み込み
        let loaded_data: Vec<TestData> = load_json(test_file.to_str().unwrap()).unwrap();

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

        let result: Result<Vec<TestData>> = load_json(test_file.to_str().unwrap());
        assert!(
            result.is_err(),
            "存在しないファイルの読み込みはエラーになるべき"
        );
    }

    #[test]
    fn test_load_json_with_invalid_json() {
        let test_file = get_test_file_path("invalid.json");

        // 不正なJSONを書き込む
        fs::write(&test_file, "{ invalid json }").unwrap();

        let result: Result<Vec<TestData>> = load_json(test_file.to_str().unwrap());
        assert!(result.is_err(), "不正なJSONの読み込みはエラーになるべき");

        // クリーンアップ
        let _ = fs::remove_file(&test_file);
    }
}
