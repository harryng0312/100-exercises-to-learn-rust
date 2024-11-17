// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    // println!("++{:p} {:p} {:p} {:p}", &v, &&v, &&&v, &&&&v);
    let v_static: &mut [i32] = v.leak();

    let (v1, v2) = v_static.split_at(v_static.len() / 2);
    let t1 = thread::spawn(|| {v1.iter().sum::<i32>()});
    let t2 = thread::spawn(|| {v2.iter().sum::<i32>()});

    // println!("--{:p} {:p} {:p}", v_static, &v_static, &&v_static);
    t1.join().unwrap() + t2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
