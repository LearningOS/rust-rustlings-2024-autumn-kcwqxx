// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![]; // 创建一个空的向量以存储线程句柄
    for i in 0..10 {// 循环10次，每次创建一个线程
        handles.push(thread::spawn(move || {
            let start = Instant::now();//记录每次开始时间
            thread::sleep(Duration::from_millis(250));//每个进程休眠250毫秒
            println!("thread {} is complete", i);//打印线程完成信息
            start.elapsed().as_millis()//返回线程运行时间
        }));
    }

    let mut results: Vec<u128> = vec![];// 创建一个空的向量以存储线程返回的结果
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        results.push(handle.join().unwrap());//将每个线程的结果添加到结果向量中
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
