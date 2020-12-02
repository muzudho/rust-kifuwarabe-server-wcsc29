use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{thread, time};

const BESTMOVE_TXT: &str = "bestmove.txt";

/// My name is Kifuwarabe server.
///
/// Let's explain how to use me.
/// 
/// Windows 10.
/// 
/// [Windows] + [R].
/// `cmd`, [Enter].
/// 
/// ```Shell
/// ### Example.
/// cd C:\muzudho\projects_rust\rust-kifuwarabe-server-wcsc29
/// cls
/// 
/// ### Compile.
/// cargo clippy
/// 
/// ### Run.
/// cargo run --release
/// ```
/// 
/// Execution file.
/// C:/muzudho/projects_rust/rust-kifuwarabe-server-wcsc29/target/release/rust-kifuwarabe-server-wcsc29.exe
fn main() {
    println!("Server started.");
    loop {
        // 次の一手ファイルの有無確認。
        if Path::new(BESTMOVE_TXT).exists() {
            // out.txt ファイルを読取に行く。別のプロセスが使用していて、エラーになることがよくある。
            let mut file;
            loop {
                match File::open(BESTMOVE_TXT) {
                    Ok(n) => {file = n; break;},
                    Err(err) => {
                        println!("File open error. {:?}", err);
                        // 0.3秒ぐらい待機してから繰り返し。
                        thread::sleep(time::Duration::from_millis(300));
                    }
                };
            }

            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(n) => n,
                Err(err) => panic!("File open error. {:?}", err),
            };

        }
    }
}
