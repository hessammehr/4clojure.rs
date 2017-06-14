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

mod problem29 {
    // Given a string, keep its capital letters:
    // "Hello World!" -> "HW"
    fn is_capital(&c: &char) -> bool {
        c >= 'A' && c <= 'Z'
    }

    pub fn capitals(s:&str) -> String {
        s.chars().filter(is_capital).collect()
    }
}

mod problem30 {
    pub fn deduped<T: PartialEq, I: Iterator<Item=T>>(mut i: I) -> Vec<T> {
        let mut res: Vec<T> = Vec::new();
        if let Some(mut old) = i.next() {
            // res.push(old);
            loop {
                match i.next() {
                    Some(new) => { if old != new { res.push(old); } old = new; }
                    None => { res.push(old); break; }
                }
            }
        }
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

    #[test]
    fn problem29() {
        use problem29::capitals;
        assert_eq!(capitals("HeLlO, WoRlD!"), "HLOWRD".to_owned());
        assert_eq!(capitals("nothing"), "".to_owned());
        assert_eq!(capitals("$#A(*&987Zf"), "AZ".to_owned());
    }

    #[test]
    fn problem30() {
        use problem30::{deduped};
        use std::iter::FromIterator;
        println!("{:?}", deduped(vec![1, 1, 2, 3, 3, 2, 2, 3].iter()));
        assert_eq!("Leroy" , String::from_iter(deduped("Leeeeeerrroyyy".chars()).into_iter()));

    }
}
