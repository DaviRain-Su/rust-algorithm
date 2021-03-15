
use queue::Queue;

fn main() {
    let mut q = Queue::new();
    println!("q = {:?}", q);
    q.push('q');
    println!("q = {:?}", q);
    let ret = q.pop();
    println!("ret = {:?}", ret);
}
