pub struct SomeBody {
    name: String,
    age: i32,
}

impl PartialEq for SomeBody {
    fn eq(&self, other: &SomeBody) -> bool {
        self.name == other.name && self.age == other.age
    }
}

fn main() {
    let st1 = SomeBody {
        name: "xiaoMing".to_string(),
        age: 22,
    };

    let st2 = SomeBody {
        name: "xiaoHone".to_string(),
        age: 23,
    };

    println!("{}", st1 == st2);
}
