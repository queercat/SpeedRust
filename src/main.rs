use std::time::{Instant};

/// Returns an approximation of Pi over n itterations using the Leibniz series.
/// 
/// ## Arguments
/// * n {u32} - The number of iterations to run.
/// 
/// ## Returns
/// {f32} - The resulting Pi approximation   
fn aproximate_pi(n: u32) -> f32 {
    let mut result : f32 = 0.0;
    let mut sign : i8 = 1;

    for i in 0..n {
        result += (sign as f32) / (2.0 * i as f32 + 1.0);
        sign = -sign;
    }

    return result * 4.0;
}

/// Runs and times the Leibniz series.
fn main() {
    let terms: i32 = 10;

    let now = Instant::now();
    let pi : f32 = aproximate_pi(terms.pow(8) as u32);

    println!("{} ms", now.elapsed().as_millis());
    println!("Pi is approx --> {}", pi);
}