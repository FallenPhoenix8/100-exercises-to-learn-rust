use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    thread::scope(|scope| {
        let mid = v.len() / 2;
        let (left_half, right_half) = v.split_at(mid);

        let left_sum = scope.spawn(|| left_half.iter().sum::<i32>());
        let right_sum = scope.spawn(|| right_half.iter().sum::<i32>());

        left_sum.join().unwrap() + right_sum.join().unwrap()
    })
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
