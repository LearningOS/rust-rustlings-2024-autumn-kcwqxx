// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.


use std::mem;//使用swap记得加上
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        println!("my_option is None!");//None无法解包
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    //let my_empty_vec让返回值给了my_empty_vec，但my_empty_vec是let_，没有实际用途
    vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("This Vec is empty, see? {:?}", ());//空元组防止报错
    
    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
   /*  let temp = value_a;
    value_a = value_b;
    value_b = temp;太繁琐，clippy建议用系统自带的swap*/
    mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
