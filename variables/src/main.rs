const _MAX_POINTS: u32 = 100_000;//声明常量使用 const 关键字，且必须声明数据类型 ,数字之间的下滑线用于增加可读性，没有特殊意义

fn main() {
    let _a: i32 = 5;
    // a = 6; //err

    // let mut a: i32 = 5;  //mut修饰的可以重新赋值
    // a = 6;

    let space: String = "     ".to_string();
    let _space: usize = space.len();

    let _value: i32 = "42".parse().unwrap();//必须声明类型 (u32或者其他类型) ,疑问：是如何根据结果的类型自动选择执行的方法
    let _value: String = "42".parse().unwrap();
}
