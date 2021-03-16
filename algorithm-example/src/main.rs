
use queue::Queue;
use sort_algorithm::selection;

fn main() {
    println!("************************** Queue *******************************");
    let mut q = Queue::new();
    println!("q = {:?}", q);
    q.push('q');
    println!("q = {:?}", q);
    let ret = q.pop();
    println!("ret = {:?}", ret);

    println!("************************** Selection ****************************");
    let v = vec![5, 4, 3, 2, 1];
    let mut tmp = selection::SelectionSort::new(v);
    println!("v sort before = {:?}", tmp);
    tmp.sort();
    println!("v sort after = {:?}", tmp);
    let v = tmp.get_vec();
    println!("v  = {:?}", v);
}
