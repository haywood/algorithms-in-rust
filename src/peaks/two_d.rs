macro_rules! assert_shape {
    ($m:expr) => {
        {
            let n = $m.len();
            if n == 0 { (0, 0) } else {
                let m = $m[0].len();
                for r in &$m[1..] {
                    if r.len() != m {
                        // silently fail on invalid input
                        return None
                    }
                }
                (n, m)
            }
        }
    };
}

// O(n*log(m))
pub fn find_peak<T: Ord + Clone>(input: &[&[T]]) -> Option<(usize, usize)> {
    let (n, m) = assert_shape!(input);
    if n == 0 || m == 0 { return None }
    let mut max = n - 1;
    let mut min = 0;
    loop {
        let mid = (min + max) / 2;
        let row = input[mid];
        let mut max_j = 0;
        let mut max_t = &row[max_j];
        for tmp_j in 0..m {
            let tmp_t = &row[tmp_j];
            if tmp_t > max_t {
                max_t = tmp_t;
                max_j = tmp_j;
            }
        }
        if mid > min && &input[mid - 1][max_j] > max_t {
            max = mid - 1;
        } else if mid < max && &input[mid + 1][max_j] > max_t {
            min = mid + 1;
        } else {
            return Some((mid, max_j))
        }
    }
}

// O(nm)
pub fn find_peak_greedy<T: Ord + Clone>(input: &[&[T]]) -> Option<(usize, usize)> {
    let (n, m) = assert_shape!(input);
    if n == 0 || m == 0 { return None }
    let mut i = n / 2;
    let mut j = m / 2;
    let mut candidates = Vec::<(usize, usize)>::with_capacity(4);
    loop {
        candidates.clear();
        if i != 0 { candidates.push((i - 1, j)); }
        if j != 0 { candidates.push((i, j - 1)); }
        if i < n - 1 { candidates.push((i + 1, j)); }
        if j < m - 1 { candidates.push((i, j + 1)); }
        let (mut max_i, mut max_j) = candidates[0];
        let mut max_t = &input[max_i][max_j];
        for &(tmp_i, tmp_j) in candidates.iter() {
            let tmp_t = &input[tmp_i][tmp_j];
            if tmp_t > max_t {
                max_t = tmp_t;
                max_i = tmp_i;
                max_j = tmp_j;
            }
        }
        if max_t > &input[i][j] {
            i = max_i;
            j = max_j;
        } else {
            return Some((i, j))
        }
    }
}

// O(nm)
pub fn find_peak_naive<T: Ord + Clone>(input: &[&[T]]) -> Option<(usize, usize)> {
    let (n, m) = assert_shape!(input);
    for i in 0..n {
        for j in 0..m {
            let ref x = input[i][j];
            if (i == 0 || &input[i - 1][j] <= x) &&
                (j == 0 || &input[i][j - 1] <= x) &&
                    (i == n - 1 || &input[i + 1][j] <= x) &&
                    (j == m - 1 || &input[i][j + 1] <= x) {
                        return Some((i, j));
                    }
        }
    }
    None
}

macro_rules! test_is_peak {
    ($algo:ident, $input:expr) => {
        {
            let input: &[&[u64]] = $input;
            let n = input.len();
            let m = input[0].len();
            match $algo(input) {
                None => panic!("failed to find peak in input {:?}", input),
                Some((i, j)) => {
                    let x = input[i][j];
                    if i != 0 { assert!(x >= input[i - 1][j]) }
                    if j != 0 { assert!(x >= input[i][j - 1]) }
                    if i != n - 1 { assert!(x >= input[i + 1][j]) }
                    if j != m - 1 { assert!(x >= input[i][j + 1]) }
                }
            }
        }
    };
}

macro_rules! test {
    ($name:ident, $algo:ident) => {
        #[test]
        fn $name() {
            assert_eq!($algo(&Vec::<&[i64]>::new()), None);

            assert_eq!($algo(&[&Vec::<i64>::new()]), None);

            test_is_peak!($algo, &[
                          &[1, 2, 3, 4],
                          &[5, 18, 20, 8],
                          &[9, 10, 19, 12],
                          &[13, 14, 15, 16]
            ]);

            test_is_peak!($algo, &[
                          &[1, 2, 3, 4],
                          &[5, 18, 7, 8],
                          &[9, 10, 19, 12],
                          &[13, 14, 15, 16]
            ]);

            test_is_peak!($algo, &[
                          &[1, 2, 3, 4],
                          &[5, 6, 7, 8],
                          &[9, 10, 17, 12],
                          &[13, 14, 15, 16]
            ]);

            test_is_peak!($algo, &[
                          &[1, 2, 3, 4],
                          &[5, 6, 7, 8],
                          &[9, 10, 11, 12],
                          &[13, 14, 15, 16]
            ]);
        }
    };
}

test! { test_find_peak, find_peak }

test! { test_find_peak_greedy, find_peak_greedy }

test! { test_find_peak_naive, find_peak_naive }
