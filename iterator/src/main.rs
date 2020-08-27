fn main() {
    let vec = vec![1, 2, 3, 4];

    //虽然创建了一个迭代器，但不会有任何改变，因为rust的迭代器是lazy的，惰性的
    let vec_iter = vec.iter();

    for val in vec_iter {
        println!("{}", val)
    }

    let map: Vec<i32> = vec.iter().map(|x| x + 1).collect();

    println!("{:?}", map);

    let counter = Counter::new(1);
    let sum: i32 = counter.zip(Counter::new(2)).map(|(x, y)| x * y).sum();

    println!("{}", sum);
}

struct Counter {
    count: i32,
}

impl Counter {
    fn new(count: i32) -> Counter {
        Counter { count }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count <= 6 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}