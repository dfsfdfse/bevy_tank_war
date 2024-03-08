use std::collections::VecDeque;
fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let mut iter = data.into_iter();

    let mut queue = VecDeque::new();

    if let Some(first) = iter.next() {
        queue.push_back(first);
    }

    for i in iter {
        for j in &queue {
            println!("({}, {})", j, i);
        }
        queue.push_back(i);
    }
}
