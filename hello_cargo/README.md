# hello_cargo

cargoを使ってhello world.

https://doc.rust-jp.rs/book/second-edition/ch01-03-hello-cargo.html

## Command

* `cargo new` -> Rustの新規プロジェクトを生成する。基本はこの方法で生成で良いらしい。標準で `git` の初期化も行う（オプションで取り消し可）
* `cargo build` -> ビルドされ、同プロジェクトの `.target/debug` 以下にバイナリが生成される。 `.target/debug/hello_cargo` で実行可能
* `cargo run` -> コンパイル&実行、 `go run` と同様 
* `cargo check` -> コンパイル可能かどうかを確認

