use crate::List::{NEXT, NULL};
use std::ops::Deref;

fn main() {
    //声明一个box
    let five = Box::new(5);
    println!("box = {}", five);

    //========================

    let list = NEXT("123", Box::new(NEXT("234", Box::new(NULL))));

    println!("{:#?}", list);

    un_reference();
}

#[derive(Debug)]
enum List<T> {
    NEXT(T, Box<List<T>>),
    NULL,
}

//自定义智能指针
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(val: T) -> MyBox<T> {
        MyBox(val)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}


fn un_reference() {
    let five = 5;
    let my_five = MyBox::new(5);
    println!("{}", five == *my_five);
}