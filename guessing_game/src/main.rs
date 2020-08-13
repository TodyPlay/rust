use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("这是一个猜数字游戏");

    let rand_num = rand::thread_rng().gen_range(1, 101);

    // println!("this random num is :{}", rand_num);

    loop {
        println!("输入一个数字，以回车结束");
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str).expect("读取失败,请重启程序");

        let num: u32;

        let result = num_str.trim().parse();

        match result {
            Ok(ok) => num = ok,
            Err(err) => {
                println!("输入的不是数字请重试 :{}", err.to_string());
                continue;
            }
        }

        println!("你输入的数字 : {}", num);

        let order = num.cmp(&rand_num);

        match order {
            Ordering::Less => println!("你输入的数字小了"),
            Ordering::Greater => println!("你输入的数字大了"),
            Ordering::Equal => {
                println!("猜中了，游戏结束");
                break;
            }
        }
    }
}
