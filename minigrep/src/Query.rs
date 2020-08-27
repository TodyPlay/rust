use std::fs::read_to_string;

pub struct Query {
    search: String,
    content: String,
}

impl Query {
    pub fn new(vec: &[String]) -> Result<Query, String> {
        if vec.len() < 3 {
            Err("入参数量不够".to_string())
        } else {
            let content = read_to_string(&vec[2]);
            match content {
                Ok(content) => Ok(Query {
                    search: vec[1].clone(),
                    content,
                }),
                Err(err) => Err(err.to_string()),
            }
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