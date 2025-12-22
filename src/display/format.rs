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
}
