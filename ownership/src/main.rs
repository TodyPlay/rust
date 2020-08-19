fn main() {
    {
        let s1 = String::from("hello");
        let s2 = s1;

        // println!("si :{} , s2 :{}", s1, s2);  //这里报错 s1无法使用，因为s1被赋值到 s2 ，为防止 内存二次释放，所以s1不能再使用 ，称为所有权移动

        handle_str(s2);

        // println!("main str :{}", s2);        //这里报错，s2无法使用，因为s2的所有权移动到了 handle_str方法中，方法会回收内存 ，所以s2不能再使用
    }

    {
        let str1 = get_a_string();

        let str = back_string(str1);

        // println!("after back_string :{}", str1); //这里报错，因为 上面的方法 str1 的所有权已经被移动了

        let _len = no_owner_ship(&str);
        println!("after no_owner_ship :{}", str); //这里可以继续使用，因为str的所有权并没有移动到方法中
    }

    {
        let mut str = String::from("hello");
        // mut_reference(&mut str);     //以下两行代码也有相同作用
        let str = &mut str;
        mut_reference(str);
        println!("mut reference :{}", str);
    }
    {
        let mut str = String::from("hello");
        // let mut1 = &mut str;
        // let mut2 = &mut str;

        // println!("{}{}", mut1, mut2); //报错，因为发生了两次可变引用

        // println!("{}{}", &mut str, &mut str); //同样报错，两次可变引用

        let mut1 = &mut str;

        println!("{}{}", mut1, mut1);
    }

    {
        let str = String::from("hello");
        let str2 = slices(&str);    //str2是一个引用str的slice

        // str.clear();     //此处创建了以一个可变引用，前面有一个不可变引用，在可变引用创建后不能使用不可变引用，所以会报错

        println!("str: {}, slice:{}", str, str2);
    }

    {
        let str = String::from("slices");
        let str1 = reference(&str);
        // str.clear(); //报错 原因同上
        println!("self:{},reference:{}", str, str1);
    }
}

fn handle_str(str: String) {
    println!("some sting :{}", str);
}

//方法的返回值会将返回值的所有权也同时返回 --首先要有所有权
fn get_a_string() -> String {
    String::from("hello")
}

//方法可以将入参直接返回，应为方法中已经拥有参数的所有权，可以将所有权返回
fn back_string(str: String) -> String {
    str
}

fn no_owner_ship(str: &String) -> usize {
    str.len()
}

fn mut_reference(str: &mut String) {
    str.push_str(" world");
}

fn slices(str: &String) -> &str {
    &str[0..1]
}

fn reference(str: &String) -> &String {
    str
}