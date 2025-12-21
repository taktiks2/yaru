use clap::{Parser, error::ErrorKind};
use yaru::{Args, run};

fn main() {
    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(e) => {
            // NOTE: clapのエラーの種類をチェック
            match e.kind() {
                ErrorKind::InvalidSubcommand => {
                    eprintln!("エラー: 無効なサブコマンドです");
                    eprintln!("使用可能なコマンド: list, add, edit, delete");
                    std::process::exit(1);
                }
                _ => {
                    // NOTE: その他のエラーはclapのデフォルトメッセージを使用
                    e.exit();
                }
            }
        }
    };

    if let Err(e) = run(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
