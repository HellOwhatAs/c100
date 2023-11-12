use std::time::Instant;
use proclib::proc_func;

fn main() {
    let start = Instant::now();
    let tmp = proc_func!("./Dictionary.txt");
    println!("{:?}", tmp);
    println!("{}", tmp.len());
    let duration = start.elapsed();
    println!("{} ms", duration.as_millis());
}