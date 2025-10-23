fn func1(s: &mut String) {
    s.push_str(" world");
}

fn func2(s: &mut String) {
    s.push_str("!");
}

fn test1<'a, 'b: 'a>(x: &'a mut Vec<&'b str>, a: &'b str) {
    x.push(a);
}

fn test2(x: &mut Vec<&str>) {
    x.push("");
}

fn test3<'a>(x: &'a mut Vec<&'a str>, a: &'a str) {
    x.push(a);
}

fn test4<'a, 'b: 'a>(x: &'a mut Vec<&'b str>, a: &'b str) {
    x.push(a);
}

#[allow(dead_code)]
fn a () {
    let mut x = vec![];
    test1(&mut x, "");
    test1(&mut x, "");
    test2(&mut x);
    test2(&mut x);
    let a = "".to_string();
    test4(&mut x, &a);
    test4(&mut x, &a);
    let b = "".to_string();
    test3(&mut x, &b);
    test4(&mut x, &b);
}

fn main() {
    let mut s = String::from("hello");

    // 调用 func1，传入 s 的可变借用
    func1(&mut s);

    // func1 调用结束后，它对 s 的借用就结束了。
    // 现在我们可以再次创建一个新的可变借用，并将其传递给 func2。
    func2(&mut s);

    println!("{}", s); // 输出: "hello world!"
}
