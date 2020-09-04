pub mod two_tree;

#[cfg(test)]
mod tests {
    use crate::two_tree::TwoTree;
    use rand::Rng;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test1() {
        let mut tree = TwoTree::new();

        let mut vec: Vec<i32> = Vec::new();

        loop {
            let rand_num = rand::thread_rng().gen_range(1, 101);
            vec.push(rand_num);
            if vec.len() == 100 {
                break;
            };
        };

        for x in vec {
            tree.add(x);
        };

        tree.for_each(|x| println!("{}", x));
    }

    #[test]
    fn test2() {
        let mut tree = TwoTree::new();
        tree.add("234");
        tree.add("123");
        tree.add("8908098");
        tree.for_each(|x| println!("{}", x));
    }
}