fn main() {
    //结构体实例化
    let mut user = User {
        username: "xiao,ming".to_string(),
        email: "@qq.com".to_string(),
        password: "123456".to_string(),
        active: true,
    };
    //结构体重新赋值
    user.email = String::from("12322@qq.com");
    //获取结构体的值
    let username = &user.username;

    //从一个结构体创建另一个结构体
    let user2 = User {
        ..user
    };

    let user3 = User {
        active: false,
        ..user2
    };
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    password: String,
    active: bool,
}

impl PartialEq<User> for User {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username && self.email == other.email && self.password == other.password && self.active == other.active
    }
}