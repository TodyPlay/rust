use std::fs::read_to_string;
use std::env::Args;

pub struct Query {
    search: String,
    content: String,
}

impl Query {
    pub fn new(mut args: Args) -> Result<Query, String> {
        args.next();

        let search = match args.next() {
            Some(val) => val,
            None => return Err("no search str".to_string()),
        };

        let filename = match args.next() {
            Some(val) => val,
            None => return Err("no file name str".to_string()),
        };

        let content = read_to_string(&filename);


        match content {
            Ok(content) => Ok(Query {
                search,
                content,
            }),
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn print_file(&self) {
        println!("{}", self.content);
    }

    pub fn search(&self) -> Vec<&str> {
        let mut vec = Vec::new();

        for line in self.content.lines() {
            if line.contains(&self.search) {
                vec.push(line);
            }
        }
        vec
    }
}