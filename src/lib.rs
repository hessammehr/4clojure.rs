mod problem27 {
    // palindrome detector for Strings
    pub fn palindrome(s: &str) -> bool {
        let c = s.chars();
        // c.collect::<Vec<_>>() == c.rev().collect::<Vec<_>>()
        c.clone().eq(c.rev())
    }
}

mod problem28 {
    // flatten for vectors
    #[derive(Debug,Clone)]
    pub enum Seq<T> {
        Item(T),
        Sequence(Vec<Seq<T>>)
    }

    pub fn flatten<T>(s: Seq<T>) -> Vec<T> {
        let mut res = vec![];
        fn inner<T>(s: Seq<T>, acc: &mut Vec<T>) {
            match s {
                Seq::Item(a) => acc.push(a),
                Seq::Sequence(ss) => { for i in ss { inner(i, acc) } },
            }
        }
        inner(s, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem27() {
        use problem27::palindrome;
        assert_eq!(true, palindrome("racecar"));
        assert_eq!(false, palindrome("adad"));
        assert_eq!(true, palindrome("人人為我,我為人人"));
        assert_eq!(true, palindrome("아들딸들아"));
        assert_eq!(false, palindrome("Я иду с мечем, судия"));
    }

    #[test]
    fn problem28() {
        use problem28::{flatten, Seq};
        // ["a", ["b", "c"], "d"]
        let s1 = Seq::Sequence(vec![Seq::Item("a"), Seq::Sequence(vec![Seq::Item("b"), Seq::Item("c")]), Seq::Item("d")]);
        println!("flatten({:?}) = {:?}", s1, flatten(s1.clone()));
        assert_eq!(flatten(s1),
                   vec!["a", "b", "c", "d"])
    }
}
