use std::fmt::Debug;

/*
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operation<T: Eq + Clone + Debug> {
    Replace(T),
    Append(T),
    Delete,
    Nop,
}
*/

pub fn levenshtein_dist<T: Eq + Clone + Debug>(src: &[T], dst: &[T]) -> usize {
    let m = src.len();
    let n = dst.len();
    let mut table = (0..=n).collect::<Vec<_>>();

    for i in 1..=m {
        let mut new_table = vec![i; n + 1];
        for j in 1..=n {
            let v = &[&new_table[j - 1], &table[j], &table[j - 1]];
            let (ind, n_el) = v
                .iter()
                .enumerate()
                .min_by_key(|e| {
                    if e.0 == 2 && src[i - 1] == dst[j - 1] {
                        **e.1
                    } else {
                        **e.1 + 1
                    }
                })
                .unwrap();

            match ind {
                0 => {
                    new_table[j] = **n_el + 1;
                }
                1 => {
                    new_table[j] = **n_el + 1;
                }
                2 => {
                    if src[i - 1] != dst[j - 1] {
                        new_table[j] = **n_el + 1;
                    } else {
                        new_table[j] = **n_el;
                    }
                }
                _ => unreachable!("not reachable"),
            }
        }
        table = new_table;
    }
    table.pop().unwrap()
}

/*
pub fn levenshtein<T: Eq + Clone + Debug>(src: &[T], dst: &[T]) -> (usize, Vec<Operation<T>>) {
    let m = src.len();
    let n = dst.len();
    let mut table = vec![vec![(0, vec![]); n + 1]; m + 1];

    for i in 1..=m {
        table[i][0] = (i, (0..i).map(|_| Operation::Delete).collect());
    }

    for j in 1..=n {
        table[0][j] = (
            j,
            (&dst[..j])
                .into_iter()
                .map(|c| Operation::Append(c.clone()))
                .collect(),
        );
    }

    for i in 1..=m {
        for j in 1..=n {
            let (ind, (mut n_el, ops)) =
                (&[&table[i][j - 1], &table[i - 1][j], &table[i - 1][j - 1]])
                    .iter()
                    .enumerate()
                    .min_by_key(|e| {
                        if e.0 == 2 && src[i - 1] == dst[j - 1] {
                            (e.1).0
                        } else {
                            (e.1).0 + 1
                        }
                    })
                    .unwrap();
            let mut ops = ops.clone();

            match ind {
                0 => {
                    n_el += 1;
                    ops.push(Operation::Append(dst[j - 1].clone()));
                }
                1 => {
                    n_el += 1;
                    ops.push(Operation::Delete);
                }
                2 => {
                    if src[i - 1] == dst[j - 1] {
                        ops.push(Operation::Nop);
                    } else {
                        n_el += 1;
                        ops.push(Operation::Replace(dst[j - 1].clone()));
                    }
                }
                _ => unreachable!("not reachable"),
            }

            table[i][j] = (n_el, ops);
        }
    }
    table.pop().unwrap().pop().unwrap()
}
*/

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_levenshtein() {
        assert_eq!(
            levenshtein_dist(
                &"acact".chars().collect::<Vec<_>>(),
                &"aaaaa".chars().collect::<Vec<_>>()
            ),
            3
        );

        assert_eq!(
            levenshtein_dist(
                &"bnac".chars().collect::<Vec<_>>(),
                &"aaaaa".chars().collect::<Vec<_>>()
            ),
            4
        );
    }
}
