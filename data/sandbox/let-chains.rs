fn main () {
    if let Some(a) = Some(Some(Some(2)))
        && let Some(b) = a
        && let Some(c) = b
    {
        println!("{:?}, {:?}, {:?}", a, b, c);
    }
}
