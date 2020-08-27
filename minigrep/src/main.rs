use std::env;
use minigrep::query::Query;
use std::env::Args;

fn main() {
    let args: Args = env::args();

    let query = match Query::new(args) {
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
