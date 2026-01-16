use std::time::*;
use std::thread::sleep;

pub fn pause(t: u64) {
    let n1 = Instant::now();
    sleep(Duration::from_secs(t));
    let n2 = Instant::now();
    let n3 = n2.duration_since(n1).as_secs();
    println!("## {n3} ##");
}
pub fn extf() {
    println!("### Stopping.. ###");
    pause(1);
    println!("### Stopped ###");
}
