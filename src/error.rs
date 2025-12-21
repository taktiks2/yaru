use thiserror::Error;

#[derive(Error, Debug)]
pub enum YaruError {
    #[error("ファイルの読み込みに失敗しました: {path}")]
    FileReadError {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("ファイルの書き込みに失敗しました: {path}")]
    FileWriteError {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("JSONの解析に失敗しました")]
    JsonParseError {
        #[from]
        source: serde_json::Error,
    },

    #[error("入出力エラーが発生しました")]
    IoError {
        #[from]
        source: std::io::Error,
    },

    #[error("無効なサブコマンドです。使用可能なコマンド: list, add, delete")]
    InvalidSubcommand,

    #[error(transparent)]
    ClapError(#[from] clap::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_file_read_error_message() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let error = YaruError::FileReadError {
            path: "test.json".to_string(),
            source: io_error,
        };
        assert!(
            error
                .to_string()
                .contains("ファイルの読み込みに失敗しました")
        );
        assert!(error.to_string().contains("test.json"));
    }

    #[test]
    fn test_file_write_error_message() {
        let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
        let error = YaruError::FileWriteError {
            path: "output.json".to_string(),
            source: io_error,
        };
        assert!(
            error
                .to_string()
                .contains("ファイルの書き込みに失敗しました")
        );
        assert!(error.to_string().contains("output.json"));
    }

    #[test]
    fn test_json_parse_error_message() {
        let invalid_json = "{ invalid json }";
        let json_error = serde_json::from_str::<serde_json::Value>(invalid_json).unwrap_err();
        let error = YaruError::JsonParseError { source: json_error };
        assert!(error.to_string().contains("JSONの解析に失敗しました"));
    }

    #[test]
    fn test_io_error_message() {
        let io_error = io::Error::new(io::ErrorKind::Other, "some error");
        let error = YaruError::IoError { source: io_error };
        assert_eq!(error.to_string(), "入出力エラーが発生しました");
    }

    #[test]
    fn test_invalid_subcommand_error_message() {
        let error = YaruError::InvalidSubcommand;
        assert_eq!(
            error.to_string(),
            "無効なサブコマンドです。使用可能なコマンド: list, add, delete"
        );
    }
}
