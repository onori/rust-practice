fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    
    // 変数を2回代入出来ない
    // x = 6;
    // println!("The value of x is: {}", x);

    // mutキーワードで可変
    let mut y = 5;
    println!("The value of y is: {}", y);
    
    y = 6;
    println!("The value of y is: {}", y);

    // 不変変数と定数の違い
    // 定数は値の型を必ず駐訳する必要がある,定数には定数式にしかセットできない
    // Rustでは定数は全て大文字アンダースコア
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    // シャドーイング
    // 同名の変数を新しく宣言出来る
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // 動作する
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);

    // Error、mutをつけても型を可変にすることは出来ない
    // let mut spaces = "   ";
    // spaces = spaces.len();

    
}
