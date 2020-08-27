use std::env;
use minigrep::query::Query;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = match Query::new(&args) {
        Ok(query) => query,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    let content = query.search();

    if content.is_empty() {
        println!("no match");
        return;
    }

    for x in content {
        println!("{}", x);
    }
}
