use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("数を当ててね");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("秘密の数字は次の通り：{}", secret_number);

    loop {
        println!("予想を入力してね");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("読み込みに失敗しました。");

        let guess: u32 = guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }
        .expect("数値を入力してね！");

        println!("次のように予想しました！：{}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎ!"),
            Ordering::Greater => println!("大きすぎ！"),
            Ordering::Equal => {
                println!("やったね！");
                break;
            },
        }
    }
    
}
