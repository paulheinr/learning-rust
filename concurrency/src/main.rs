#[allow(unused_imports)]
use std::thread;

#[allow(unused_imports)]
use easybench::bench;

pub mod mpi;
pub mod mpi_01;

#[allow(unused_variables)]
fn main() {
    let upper = 1_000_000;
    let num_threads = 100;
    //mpi_01::print_rank_number();
    //mpi_01::print_number_per_rank();
    mpi_01::blocking()
    //println!("No parallel execution needed: {}", bench(|| no_parallel_sum(upper)));
    //println!("Parallel execution of {num_threads} threads needed: {}", bench(|| parallel_sum_complex(upper, num_threads)));
}

#[allow(dead_code)]
fn no_parallel_sum(upper: i32) -> i64 {
    let mut sum: i64 = 0;
    for i in 1..upper {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i as i64;
        }
    }
    // println!("Sum from 1..{} is {}", upper, sum);
    sum
}

#[allow(dead_code)]
fn parallel_sum_simple(upper: i32) -> i64 {
    let handle1 = thread::spawn(move || {
        let mut sum: i64 = 0;
        for i in 1..upper / 2 {
            if i % 3 == 0 || i % 5 == 0 {
                sum += i as i64;
            }
        }
        sum
    });

    let handle2 = thread::spawn(move || {
        let mut sum: i64 = 0;
        for i in upper / 2..upper {
            if i % 3 == 0 || i % 5 == 0 {
                sum += i as i64;
            }
        }
        sum
    });
    handle1.join().unwrap() + handle2.join().unwrap()
}

#[allow(dead_code)]
fn parallel_sum_complex(upper: i32, num_threads: i32) -> i64 {
    let mut threads = vec![];
    for t in 0..num_threads {
        let begin = (upper / num_threads) * t;
        let end = (upper / num_threads) * (t + 1);
        threads.push(thread::spawn(move || {
            let mut sum: i64 = 0;
            for i in begin..end {
                if i % 3 == 0 || i % 5 == 0 {
                    sum += i as i64;
                }
            }
            sum
        }));
    }
    let s = threads.into_iter().map(|t| t.join().unwrap()).sum();
    //println!("sum is {s}");
    s
}
