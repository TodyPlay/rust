use futures::executor::block_on;

fn main() {
    block_on(async_main());
}


async fn learn_song() {
    println!("learn song");
}

async fn sing_song() {
    println!("sing song");
}

async fn dance() {
    println!("dance");
}

async fn learn_and_sing() {
    learn_song().await;
    sing_song().await;
}

async fn async_main() {
    dance().await;
    learn_and_sing().await;
}