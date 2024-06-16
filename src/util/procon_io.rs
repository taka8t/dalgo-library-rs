#![allow(unused_imports, unused_macros)]

#[macro_export]
macro_rules! input {
    (iter = $iter:expr, $($r:tt)*) => {
        input_inner!{$iter, $($r)*}
    };
    (buf = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

#[macro_export]
macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};
    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read!($iter, $t);
        input_inner!{$iter $($r)*}
    };
    ($iter:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

#[macro_export]
macro_rules! read {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read!($iter, $t)),* )
    };
    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read!($iter, $t)).collect::<Vec<_>>()
    };
    ($iter:expr, chars) => {
        read!($iter, String).chars().collect::<Vec<char>>()
    };
    ($iter:expr, bytes) => {
        read!($iter, String).bytes().collect::<Vec<u8>>()
    };
    ($iter:expr, usize1) => {
        read!($iter, usize) - 1
    };
    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}