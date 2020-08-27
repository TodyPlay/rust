use std::thread;
use std::time::Duration;

fn main() {
    //声明一个闭包
    let slow_num = |num: u32| {
        println!("thread sleep 3s");
        thread::sleep(Duration::from_secs(3));
        num + 1
    };

    num_add2(slow_num(12));

    let mut cache = Cacher::new(slow_num);

    let value = cache.value(23);

    println!("{:#?}", value)
}

fn num_add2(num: u32) {
    println!("{}", num + 2);
}

struct Cacher<T>
    where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
            Some(v) => v,
        }
    }
}