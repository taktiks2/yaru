use crate::application::dto::task_dto::TagInfo;
use chrono::{DateTime, Local, NaiveDate, Utc};

/// UTC時間を現地時間に変換してフォーマット
pub fn format_local_time(utc_time: &DateTime<Utc>) -> String {
    utc_time
        .with_timezone(&Local)
        .format("%Y-%m-%d %H:%M")
        .to_string()
}

/// 日付をフォーマット
///
/// # 引数
/// - `date`: フォーマットする日付（Option型）
///
/// # 戻り値
/// - 日付が存在する場合: "YYYY-MM-DD" 形式の文字列
/// - 日付が存在しない場合: "-"
pub fn format_date(date: &Option<NaiveDate>) -> String {
    date.map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "-".to_string())
}

/// 空文字列を"-"に変換
///
/// # 引数
/// - `text`: チェックする文字列
///
/// # 戻り値
/// - 空文字列の場合: "-"
/// - それ以外の場合: 元の文字列
pub fn format_optional_text(text: &Option<String>) -> String {
    text.as_ref()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "-".to_string())
}

/// Option<DateTime<Utc>>をフォーマット
///
/// # 引数
/// - `dt`: フォーマットするDateTime（Option型）
///
/// # 戻り値
/// - DateTimeが存在する場合: "YYYY-MM-DD HH:MM" 形式の文字列
/// - DateTimeが存在しない場合: "-"
pub fn format_optional_datetime(dt: &Option<DateTime<Utc>>) -> String {
    dt.as_ref()
        .map(format_local_time)
        .unwrap_or_else(|| "-".to_string())
}

/// タグ情報リストを表示用文字列に変換
///
/// # 引数
/// - `tags`: タグ情報のスライス
/// - `separator`: タグ名を結合する区切り文字
///
/// # 戻り値
/// - タグが空の場合: "-"
/// - それ以外の場合: タグ名を区切り文字で結合した文字列
pub fn format_tags(tags: &[TagInfo], separator: &str) -> String {
    if tags.is_empty() {
        "-".to_string()
    } else {
        tags.iter()
            .map(|tag| tag.name.clone())
            .collect::<Vec<_>>()
            .join(separator)
    }
}

/// 説明文を指定された最大長に切り詰める
///
/// # 引数
/// - `desc`: 切り詰める説明文
/// - `max_len`: 最大文字数
///
/// # 戻り値
/// 切り詰められた文字列。元の文字列が最大長以下の場合はそのまま返す。
/// 切り詰めた場合は末尾に "..." を追加する。
/// 改行はスペースに置き換えられ、複数の連続するスペースは1つにまとめられる。
pub fn truncate_text(desc: &str, max_len: usize) -> String {
    // 複数の連続するスペースを1つにまとめる(改行も含めて)
    let normalized = desc.split_whitespace().collect::<Vec<&str>>().join(" ");

    // 文字数を確認して切り詰める
    if normalized.chars().count() > max_len {
        format!(
            "{}...",
            normalized.chars().take(max_len).collect::<String>()
        )
    } else {
        normalized
    }
}
