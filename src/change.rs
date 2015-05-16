pub fn make_change(amount: i64, coins: &[i64]) -> Vec<i64> {
    if amount > 0 {
        let mut change = make_change_impl(amount, coins);
        change.reverse();
        change
    } else {
        vec![]
    }
}

pub fn ways_to_make_change(amount: i64, coins: &[i64]) -> Vec<Vec<i64>> {
    if amount > 0 {
        let mut ways = vec![];
        for mut way in ways_to_make_change_impl(amount, coins) {
            way.reverse();
            ways.push(way);
        }
        ways
    } else {
        vec![]
    }
}

fn make_change_impl(amount: i64, coins: &[i64]) -> Vec<i64> {
    assert!(amount > 0);
    if coins.is_empty() {
        vec![]
    } else {
        let x = coins[0];
        match amount - x {
            r if r < 0 => {
                make_change_impl(amount, &coins[1..])
            },
            r if r > 0 => {
                let mut change = make_change_impl(r, coins);
                if !change.is_empty() {
                    change.push(x);
                }
                change
            },
            _ => {
                vec![x]
            }
        }
    }
}

fn ways_to_make_change_impl(amount: i64, coins: &[i64]) -> Vec<Vec<i64>> {
    assert!(amount > 0);
    if coins.is_empty() {
        vec![]
    } else {
        let x = coins[0];
        match amount - x {
            r if r < 0 => {
                ways_to_make_change_impl(amount, &coins[1..])
            },
            r => {
                let mut ways = if r > 0 {
                    let mut tmp = vec![];
                    for mut way in ways_to_make_change_impl(r, coins) {
                        way.push(x);
                        tmp.push(way);
                    }
                    tmp
                } else {
                    vec![vec![x]]
                };
                let mut tmp = ways_to_make_change_impl(amount, &coins[1..]);
                loop {
                    match tmp.pop() {
                        None => return ways,
                        Some(way) => ways.push(way)
                    }
                }
            }
        }
    }
}

macro_rules! assert_ways {
    ($amount:expr, $count:expr, $coins:expr) => {
        {
            let ways = ways_to_make_change($amount, &$coins);
            assert_eq!(ways.len(), $count);
            let mut unique_ways = ::std::collections::HashSet::new();
            for w in ways {
                let mut sum = 0;
                for coin in w.iter() {
                    sum += *coin;
                }
                assert_eq!(sum, $amount);
                unique_ways.insert(w);
            }
            assert_eq!(unique_ways.len(), $count);
        }
    };
}

#[test]
fn test_make_change() {
    assert_eq!(make_change(100, &[25, 10, 5, 1]), [25, 25, 25, 25]);
    assert_eq!(make_change(99, &[25, 10, 5]), []);
    assert_eq!(make_change(95, &[25, 10, 5]), [25, 25, 25, 10, 10]);
}

#[test]
fn test_ways_to_make_change() {
    let coins = [25, 10, 5, 1];
    assert_ways!(100, 242, coins);
    assert_ways!(99, 0, coins[..3]);
    assert_ways!(25, 13, coins);
    assert_ways!(25, 13, coins);
}
