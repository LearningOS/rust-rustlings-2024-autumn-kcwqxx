// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

//报错原因：option不是可以迭代的集合，不可以用for循环在option中迭代
fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) =option {
        res += x;
    }
    println!("{}", res);
}
