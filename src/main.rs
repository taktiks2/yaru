use yaru::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("エラー: {:?}", e);
        std::process::exit(1);
    }
}
