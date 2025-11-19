#[macro_export]
macro_rules! concat_fields {
    (var $y:ident; $($t:tt)*) => {
        let mut x = Vec::new();
        concat_fields!(@v x, @k $y; $($t)*)
    };
    (@v $x:ident, @k $y:ident; $a:ident; $($t:tt)*) => {
        $x.push(format!("{}={}", stringify!($a), $y.$a));
        concat_fields!(@v $x, @k $y; $($t)*)
    };
    (@v $x:ident, @k $y:ident; $a:ident = $b:ident; $($t:tt)*) => {
        $x.push(format!("{}={}", stringify!($a), $y.$b));
        concat_fields!(@v $x, @k $y; $($t)*)
    };
    (@v $x:ident, @k $y:ident; $($t:tt)*) => {
        $x.join(" ")
    };
}
