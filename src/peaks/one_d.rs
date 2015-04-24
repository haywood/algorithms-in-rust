// O(log(n))
pub fn find_peak<T: Ord + Clone>(input: &[T]) -> Option<usize> {
    let n = input.len();
    if n == 0 { return None };
    let mut min = 0;
    let mut max = n - 1;
    loop {
        let mid = (min + max) / 2;
        let ref x = input[mid];
        if mid > min && &input[mid - 1] > x {
            max = mid - 1;
        } else if mid < max && &input[mid + 1] > x {
            min = mid + 1
        } else {
            return Some(mid)
        }
    }
}

// O(n)
pub fn find_peak_naive<T: Ord + Clone>(input: &[T]) -> Option<usize> {
    let n = input.len();
    for i in 0..n {
        let ref x = input[i];
        if (i == 0 || &input[i - 1] <= x) &&
            (i == n - 1 || &input[i + 1] <= x) {
                return Some(i)
        }
    }
    None
}

macro_rules! test_is_peak {
    ($algo:ident, $input:expr) => {
        {
            let input = $input;
            match $algo(input) {
                None => panic!("failed to find peak in input {:?}", input),
                Some(i) => {
                    let n = input.len();
                    let x = input[i];
                    if i > 0 { assert!(x >= input[i - 1]) }
                    if i < n - 1 { assert!(x >= input[i + 1]) }
                }
            }
        }
    };
}

macro_rules! test {
    ($name:ident, $algo:ident) => {
        #[test]
        fn $name() {
            assert_eq!($algo(&Vec::<i64>::new()), None);
            test_is_peak!($algo, &[1, 1, 1, 1]);
            test_is_peak!($algo, &[1, 2, 3, 4]);
            test_is_peak!($algo, &[1, 2, 1, 4]);
            test_is_peak!($algo, &[5, 2, 1, 4]);
        }
    }
}

test! { test_find_peak, find_peak }

test! { test_find_peak_naive, find_peak_naive }
