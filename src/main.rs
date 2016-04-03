use std::cmp::Ordering;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut vect: Vec<i32> = Vec::new();

    for line in lines {
        let line = line.expect("Failed to read line");
        let val = line.parse().expect("Must enter an integer");
        vect.push(val);
    }

    for val in &vect {
        println!("before: {}", val);
    }

    qsort(&mut vect[..]);

    for val in &vect {
        println!("after: {}", val);
    }
}

fn qsort(slice: &mut [i32]) {
    if slice.len() <= 1 {
        return
    }
    let partition_idx = partition(slice);
    qsort(&mut slice[..partition_idx]);
    qsort(&mut slice[partition_idx + 1..]);
}

fn partition(slice: &mut [i32]) -> usize {
    /* Partition the slice around an element and return the idx of that
       element in linear time.
    */
    let mut partition_idx = 0;
    for i in 1..slice.len() {
        match slice[i].cmp(&slice[partition_idx]) {
            Ordering::Less => {
                slice.swap(i, partition_idx + 1);
                slice.swap(partition_idx, partition_idx + 1);
                partition_idx += 1;
            }
            _ => ()
        }
    }
    return partition_idx;
}

#[test]
fn test_qsort() {
    let mut lhs = [2, 1, 4, 3];
    qsort(&mut lhs);
    assert_eq!(lhs, [1, 2, 3, 4]);
}

#[test]
fn test_qsort_already_sorted() {
    let mut lhs = [1, 2, 3, 4];
    qsort(&mut lhs);
    assert_eq!(lhs, [1, 2, 3, 4]);
}

#[test]
fn test_qsort_all_same() {
    let mut lhs = [1, 1, 1, 1];
    qsort(&mut lhs);
    assert_eq!(lhs, [1, 1, 1, 1]);
}

#[test]
fn test_qsort_one_elem() {
    let mut lhs = [1];
    qsort(&mut lhs);
    assert_eq!(lhs, [1]);
}

#[test]
fn test_qsort_no_elem() {
    let mut lhs = [];
    qsort(&mut lhs);
    assert_eq!(lhs, []);
}
