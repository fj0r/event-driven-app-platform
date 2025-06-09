use std::collections::HashMap;

fn main () {
    let mut x = HashMap::new();
    for i in 1..10 {
        x.insert(i.to_string(), i.to_string());

    }
    println!("{:?}", &x.keys().len());
    println!("{:?}", &x);
    x.insert("3".into(), "xxx".into());
    x.insert("3".into(), "_".into());
    println!("{:?}", &x);

}
