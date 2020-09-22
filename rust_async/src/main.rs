use futures::executor::block_on;
use std::fs;

fn main() {
    block_on(async_main());
}


async fn learn_song() {
    println!("learn song");
}

async fn sing_song() {
    println!("sing song");
    let str = fs::read_to_string("C:\\Users\\83779\\Desktop\\SD-TY\\SD_TY (1).bpmn");//尝试阻塞
    match str {
        Ok(str) => println!("{}", str),
        Err(err) => println!("{}", err.to_string())
    }
}

async fn dance() {
    println!("dance");
}

async fn learn_and_sing() {
    learn_song().await;
    sing_song().await;
}

async fn async_main() {
    futures::join!(learn_and_sing(),dance());
}