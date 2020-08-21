//定义枚举
#[derive(Debug)]
enum IpAddrKind {
    V4(IpAddrV4),
    V6(IpAddrV6),
    OTHER(u8, u8, u8, u8),
}

#[derive(Debug)]
struct IpAddrV4 {
    location: (u8, u8, u8, u8)
}

#[derive(Debug)]
struct IpAddrV6 {
    location: (u8)
}

fn main() {
    let ipv4 = IpAddrKind::V4(IpAddrV4 {
        location: (192, 168, 1, 1)
    });

    println!("{:#?}", ipv4);

    //match 匹配模式
    match &ipv4 {
        IpAddrKind::V4(v4) => {
            println!("{:#?}", v4);
        }
        IpAddrKind::V6(v6) => {
            println!("{:#?}", v6);
        }
        _ => {
            println!("OTHER")
        }
    }

    //if let 控制流
    if let IpAddrKind::OTHER(_, _, _, _) = &ipv4 {
        println!("OTHER");
    }
}