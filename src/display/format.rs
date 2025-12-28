use chrono::{DateTime, Local};

/// UTC時間の文字列を現地時間に変換してフォーマット
pub fn format_local_time(utc_time_str: &str) -> String {
    DateTime::parse_from_rfc3339(utc_time_str)
        .map(|dt| {
            dt.with_timezone(&Local)
                .format("%Y-%m-%d %H:%M")
                .to_string()
        })
        .unwrap_or_else(|_| utc_time_str.to_string())
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_format_local_time_valid_rfc3339() {
        let utc_time = "2024-01-15T10:30:00Z";
        let result = format_local_time(utc_time);

        // フォーマットが "YYYY-MM-DD HH:MM" の形式であることを確認
        assert!(result.len() == 16);
        assert!(result.contains("-"));
        assert!(result.contains(":"));
    }

    #[test]
    fn test_format_local_time_with_timezone() {
        let utc_time = "2024-01-15T10:30:00+09:00";
        let result = format_local_time(utc_time);

        // フォーマットが "YYYY-MM-DD HH:MM" の形式であることを確認
        assert!(result.len() == 16);
    }

    #[test]
    fn test_format_local_time_invalid_format() {
        let invalid_time = "invalid-time-string";
        let result = format_local_time(invalid_time);

        // 無効な時刻文字列の場合は、元の文字列をそのまま返す
        assert_eq!(result, invalid_time);
    }

    #[test]
    fn test_format_local_time_empty_string() {
        let empty_time = "";
        let result = format_local_time(empty_time);

        // 空文字列の場合は、空文字列を返す
        assert_eq!(result, empty_time);
    }

    #[test]
    fn test_format_local_time_current_time() {
        let now = Utc::now().to_rfc3339();
        let result = format_local_time(&now);

        // 現在時刻をフォーマットできることを確認
        assert!(result.len() == 16);
        assert!(result.contains("202")); // 2020年代であることを確認
    }

    #[test]
    fn test_truncate_description_short() {
        // 短い説明文はそのまま返される
        let desc = "短い説明";
        let result = truncate_text(desc, 30);
        assert_eq!(result, "短い説明");
    }

    #[test]
    fn test_truncate_description_long() {
        // 長い説明文は切り詰められる
        let desc = "これは非常に長い説明文です。この説明文は30文字を超えているため切り詰められるはずです。";
        let result = truncate_text(desc, 30);
        assert_eq!(result.chars().count(), 33); // 30文字 + "..."
        assert!(result.ends_with("..."));
        assert!(result.starts_with("これは非常に長い説明文です。"));
    }

    #[test]
    fn test_truncate_description_exactly_max() {
        // ちょうど最大長の説明文
        let desc = "1234567890123456789012345678901234567890"; // 40文字
        let result = truncate_text(desc, 40);
        assert_eq!(result, desc);
    }

    #[test]
    fn test_truncate_text_with_newline() {
        // 改行を含む短い文字列は改行がスペースに置き換えられる
        let desc = "最初の行\n2行目\n3行目";
        let result = truncate_text(desc, 50);
        assert!(!result.contains('\n'));
        assert!(result.contains("最初の行 2行目 3行目"));
    }

    #[test]
    fn test_truncate_text_with_newline_and_truncate() {
        // 改行を含む長い文字列は改行がスペースに置き換えられてから切り詰められる
        let desc = "これは長い説明です\nこれは2行目で非常に長い文章が続きます\n3行目もあります";
        let result = truncate_text(desc, 30);
        assert!(!result.contains('\n'));
        assert_eq!(result.chars().count(), 33); // 30文字 + "..."
        assert!(result.ends_with("..."));
    }

    #[test]
    fn test_truncate_text_with_multiple_spaces() {
        // 複数の連続するスペースは1つにまとめられる
        let desc = "複数の    スペース   がある   テキスト";
        let result = truncate_text(desc, 50);
        assert!(!result.contains("  ")); // 連続するスペースがないことを確認
        assert_eq!(result, "複数の スペース がある テキスト");
    }

    #[test]
    fn test_truncate_text_with_newline_only() {
        // 改行のみの場合
        let desc = "\n\n\n";
        let result = truncate_text(desc, 30);
        assert!(!result.contains('\n'));
        assert_eq!(result, "");
    }
}
