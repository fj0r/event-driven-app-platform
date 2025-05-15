
fn main () -> Result<(), Box<dyn std::error::Error>> {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let len = a.len();
    let step = 5;
    let mut idx = 0;
    while idx < len / step {
        let s = idx * step;
        println!("{:?}", a[s..s+step].to_vec());
        idx += 1;
    }

    Ok(())
}
