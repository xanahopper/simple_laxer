pub mod laxer;

#[cfg(test)]
mod test {
    use crate::laxer::Laxer;

    #[test]
    fn test_demo() {
        let source = "int Xana = 10\n";
        let l = Laxer::new(source, false);
        for token in l {
            println!("{:?}", token)
        }
    }
}