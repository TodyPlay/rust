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

    //元组结构体实例化
    let mut color = Color(0, 0, 0);
    let point = Point(12, 23);
    //元组结构体取值
    let r = &color.0;
    color.1 = 12;
}

//普通结构体
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    password: String,
    active: bool,
}

//为结构体实现某个特征
impl PartialEq<User> for User {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username && self.email == other.email && self.password == other.password && self.active == other.active
    }
}

//元组结构体
struct Color(i32, i32, i32);

struct Point(i32, i32);