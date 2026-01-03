use crate::application::dto::tag_dto::TagDTO;
use crate::interface::cli::display::format::{format_local_time, format_optional_text, truncate_text};
use comfy_table::{presets::UTF8_FULL, Table};

/// タグのテーブルを作成
///
/// # 引数
/// - `tags`: 表示するタグDTOのスライス
///
/// # 戻り値
/// フォーマットされたテーブル
pub fn create_tag_table(tags: &[TagDTO]) -> Table {
    let headers = vec!["ID", "名前", "説明", "作成日", "更新日"];

    let rows: Vec<Vec<String>> = tags.iter().map(create_tag_row).collect();

    build_table_with_preset(headers, rows)
}

/// タグの詳細テーブルを作成
pub fn create_tag_detail_table(tag: &TagDTO) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    table.add_row(vec!["ID", &tag.id.to_string()]);
    table.add_row(vec!["名前", &tag.name]);
    table.add_row(vec!["説明", &format_optional_text(&tag.description)]);
    table.add_row(vec!["作成日", &format_local_time(&tag.created_at)]);
    table.add_row(vec!["更新日", &format_local_time(&tag.updated_at)]);

    table
}

/// テーブルの基本構造を作成し、行データを追加
///
/// # 引数
/// - `headers`: テーブルのヘッダー
/// - `rows`: テーブルの行データ
///
/// # 戻り値
/// UTF8_FULLプリセットが適用されたテーブル
fn build_table_with_preset(headers: Vec<&str>, rows: Vec<Vec<String>>) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(headers);

    table.add_rows(rows);

    table
}

/// タグの1行分のデータを作成
///
/// # 引数
/// - `tag`: タグDTO
///
/// # 戻り値
/// タグの1行分のデータ（文字列のベクタ）
fn create_tag_row(tag: &TagDTO) -> Vec<String> {
    let description = truncate_text(&format_optional_text(&tag.description), 20);

    vec![
        tag.id.to_string(),
        truncate_text(&tag.name, 20),
        description,
        format_local_time(&tag.created_at),
        format_local_time(&tag.updated_at),
    ]
}
