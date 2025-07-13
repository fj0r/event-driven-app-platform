#[cfg(test)]
mod test {
    use proc::Hello;
    #[test]
    fn a() {
        #[derive(Debug, Hello)]
        struct A;

        println!("{:?}", A);
        assert!(true);
    }
}

fn main() {}
