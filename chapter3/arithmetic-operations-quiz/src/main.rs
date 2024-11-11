use rand::Rng;

fn main() {
    // 正解数を保持する変数
    let mut num_of_correct = 0;

    while num_of_correct < 3 {
        // 加算
        let op1 = rand::thread_rng().gen_range(0..100);
        let op2 = rand::thread_rng().gen_range(0..100);
        println!("{} + {} = ??", op1, op2);
        println!("?? の値を入力してください:");
        // ユーザーからの回答を保持する変数
        let mut ans_input = String::new();
        // 標準入力から1行取得し,ans_inputに代入
        std::io::stdin().read_line(&mut ans_input).unwrap();
        // ans_inputからtrim()で改行を取り除き,parse()で整数に変換
        let ans_input = ans_input.trim().parse::<i32>().unwrap();

        dbg!(ans_input);
        if dbg!(ans_input == op1 + op2) {
            println!("正解!");
            num_of_correct += 1;
            // 3問正解したら終了
            if num_of_correct >= 3 {
                break;
            }
        } else {
            println!("不正解!");
        }

        // 減算
        let op1 = rand::thread_rng().gen_range(0..100);
        let op2 = rand::thread_rng().gen_range(0..100);
        println!("{} - {} = ??", op1, op2);
        println!("?? の値を入力してください:");

        // ユーザーからの回答を保持する変数
        let mut ans_input = String::new();

        // 標準入力から1行取得し,ans_inputに代入
        std::io::stdin().read_line(&mut ans_input).unwrap();

        // ans_inputからtrim()で改行を取り除き,parse()で整数に変換
        let ans_input = ans_input.trim().parse::<i32>().unwrap();

        dbg!(ans_input);
        if dbg!(ans_input == op1 - op2) {
            println!("正解!");
            num_of_correct += 1;
        } else {
            println!("不正解!");
        }
    }
    println!("クリア!");
}
