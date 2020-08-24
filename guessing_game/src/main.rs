// 外部依存のクレートを呼び込む use randと同様の効果
// cargo doc --open で依存クレートのライブラリが開く
extern crate rand;

// 標準(std)ライブラリにあるio（入出力）ライブラリ
use std::io;
// Rngトレイトは乱数生成機が実装するメソッドを低木々していて、このトレイトがないと、メソッドを使用出来ない
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!"); // 数を当ててごごらん

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    loop {

        println!("Please input your guess."); // ほら、予想を書いてね

        // let mut は可変可能なguess変数
        // ::はString型の関連関数、静的メソッド(Static)と同等
        // = 新たにカラのStringオブジェクトに束縛されるguess変数を作った
        let mut guess = String::new();

        // use std::io と記述してなければ、 std::io::stdin() と記述
        // read_lineは標準入力から文字列に格納、受け取る引数は可変である必要あり
        // &は参照で標準は不変、故に&mut guess と書いて可変にする必要がある
        // read_lineの戻り値はio::Result Result型は列挙型でOkかErrを持つ。
        // Errが起こった際、expectはプログラムをクラッシュさせ、引数の文字を出力
        // Okの場合、expectはそのままOkが列挙する値を返す
        io::stdin().read_line(&mut guess).expect("Failed to read line"); // 行の読み込みに失敗しました

        // 変数のシャドーイング、同変数名を再利用
        // 式中のguessはもともとのString型のguessを指す、trimメソッドで空白を削除、parseはstringを解析して何らかの数字に置き換えたりなど
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");                 //数値を入力してください！

        // match式を通じてResult型を振り分ける
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _は包括値、OK(num)にマッチしない全てをErr値にマッチ
            // 文字→数字に変換出来ないものはcontinueで再度入力を促す
            Err(_) => continue,
        };

        // {} はprintln!のマクロプレースホルダー
        println!("You guessed: {}", guess); // 次のように予想しました: {}

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            // 同じ数字だった場合、抜ける
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
