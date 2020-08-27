use std::{env, process};
use minigrep::Query;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = match Query::new(&args) {
        Ok(query) => query,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
    let content = query.search();
    for x in content {
        println!("{}", x);
    }
}
