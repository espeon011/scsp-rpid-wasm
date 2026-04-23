use crate::scsp::{ScspInstance, ScspSolution};
use rpid::prelude::{CabsParameters, SearchParameters};
use rpid::solvers::Search;
use rpid::{Bound, Dominance, Dp};
use std::collections::HashSet;

type BoundTable2 = Vec<Vec<Vec<Vec<i32>>>>;
type BoundTable3 = Vec<Vec<Vec<Vec<Vec<Vec<i32>>>>>>;

const BOUND3_MAGIC: usize = 8;

#[derive(Clone)]
pub struct ModelRpid<T: Eq + Copy + std::hash::Hash> {
    pub instance: ScspInstance<T>,
    pub bound_table2: BoundTable2,
    pub bound_table3: BoundTable3,
}

fn scs2len<T: Eq>(s1: &[T], s2: &[T]) -> Vec<Vec<i32>> {
    let len1 = s1.len();
    let len2 = s2.len();

    let mut dp = vec![vec![(len1 + len2) as i32; len2 + 1]; len1 + 1];

    // dp[i1][len2] = len1 - i1
    for (i1, dprow) in dp.iter_mut().enumerate() {
        dprow[len2] = (len1 - i1) as i32;
    }
    // dp[len1][i2] = len2 - i2
    for (i2, dpcell) in dp[len1].iter_mut().enumerate() {
        *dpcell = (len2 - i2) as i32;
    }

    for i1 in (0..=len1).rev() {
        for i2 in (0..=len2).rev() {
            if i1 == len1 || i2 == len2 {
                continue;
            }

            if s1[i1] == s2[i2] {
                dp[i1][i2] = dp[i1 + 1][i2 + 1] + 1;
            } else {
                dp[i1][i2] = i32::min(dp[i1 + 1][i2], dp[i1][i2 + 1]) + 1;
            }
        }
    }

    dp
}

fn scs3len<T: Eq + Copy + std::hash::Hash>(s1: &[T], s2: &[T], s3: &[T]) -> Vec<Vec<Vec<i32>>> {
    let len1 = s1.len();
    let len2 = s2.len();
    let len3 = s3.len();

    let mut dp = vec![vec![vec![(len1 + len2 + len3) as i32; len3 + 1]; len2 + 1]; len1 + 1];

    // dp[i1][len2][len3] = len1 - i1
    for (i1, dpv) in dp.iter_mut().enumerate() {
        dpv[len2][len3] = (len1 - i1) as i32;
    }
    // dp[len1][i2][len3] = len2 - i2
    for (i2, dpv) in dp[len1].iter_mut().enumerate() {
        dpv[len3] = (len2 - i2) as i32;
    }
    // dp[len1][len2][i3] = len3 - i3
    for (i3, dpv) in dp[len1][len2].iter_mut().enumerate() {
        *dpv = (len3 - i3) as i32;
    }

    for i1 in (0..=len1).rev() {
        for i2 in (0..=len2).rev() {
            for i3 in (0..=len3).rev() {
                if [i1 == len1, i2 == len2, i3 == len3]
                    .iter()
                    .filter(|&&cond| cond)
                    .count()
                    >= 2
                {
                    continue;
                }

                let mut fronts = HashSet::with_capacity(3);
                if i1 < len1 {
                    fronts.insert(s1[i1]);
                }
                if i2 < len2 {
                    fronts.insert(s2[i2]);
                }
                if i3 < len3 {
                    fronts.insert(s3[i3]);
                }
                let pretransversal: Vec<(usize, usize, usize)> = fronts
                    .iter()
                    .map(|&c| {
                        (
                            if i1 < len1 && s1[i1] == c { i1 + 1 } else { i1 },
                            if i2 < len2 && s2[i2] == c { i2 + 1 } else { i2 },
                            if i3 < len3 && s3[i3] == c { i3 + 1 } else { i3 },
                        )
                    })
                    .collect();
                let min_length = pretransversal
                    .iter()
                    .map(|&(pre_i1, pre_i2, pre_i3)| dp[pre_i1][pre_i2][pre_i3])
                    .min()
                    .unwrap();

                dp[i1][i2][i3] = min_length + 1;
            }
        }
    }

    dp
}

fn bound_table2<T: Eq + Copy + std::hash::Hash>(instance: &ScspInstance<T>) -> BoundTable2 {
    let mut bound_table = Vec::new();

    for (idx1, s1) in instance.seqs.iter().enumerate() {
        let mut bound = Vec::with_capacity(idx1);
        for (_idx2, s2) in instance.seqs.iter().enumerate().take(idx1) {
            bound.push(scs2len(s1, s2));
        }
        bound_table.push(bound);
    }

    bound_table
}

fn bound_table3<T: Eq + Copy + std::hash::Hash>(instance: &ScspInstance<T>) -> BoundTable3 {
    let mut bound_table = Vec::new();
    for (idx1, s1) in instance.seqs.iter().enumerate().take(BOUND3_MAGIC) {
        let mut bound_row = Vec::with_capacity(idx1);
        for (idx2, s2) in instance.seqs.iter().enumerate().take(idx1) {
            let mut bound = Vec::with_capacity(idx2);
            for (_idx3, s3) in instance.seqs.iter().enumerate().take(idx2) {
                bound.push(scs3len(s1, s2, s3));
            }
            bound_row.push(bound);
        }
        bound_table.push(bound_row);
    }

    bound_table
}

impl<T: Eq + Copy + std::hash::Hash + Default> ModelRpid<T> {
    pub fn new(instance: &ScspInstance<T>) -> Self {
        let mut instance = instance.clone();
        instance
            .seqs
            .sort_by_key(|seq| std::cmp::Reverse(seq.len()));

        Self {
            bound_table2: bound_table2(&instance),
            bound_table3: bound_table3(&instance),
            instance,
        }
    }

    pub fn solve(&self, time_limit: u32) -> ScspSolution<T> {
        if self.instance.seqs.is_empty() {
            return ScspSolution {
                seq: Some(vec![]),
                bound: Some(0),
            };
        }
        if self.instance.seqs.len() == 1 {
            return ScspSolution {
                seq: Some(self.instance.seqs[0].clone()),
                bound: Some(self.instance.seqs[0].len() as i32),
            };
        }

        let search_param = SearchParameters::<i32> {
            primal_bound: Some(self.instance.seqs.iter().map(|seq| seq.len() as i32).sum()),
            dual_bound: self.instance.seqs.iter().map(|seq| seq.len() as i32).max(),
            quiet: false,
            time_limit: Some(time_limit as f64 * 60.),
            ..Default::default()
        };
        let cabs_param = CabsParameters::default();
        let mut solver = rpid::solvers::create_cabs(self.clone(), search_param, cabs_param);
        let solution = solver.search();

        ScspSolution {
            seq: if solution.is_infeasible {
                None
            } else {
                Some(solution.transitions)
            },
            bound: solution.best_bound,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ScspState {
    indices: Vec<usize>,
    sol_len: i32,
}

impl<T: Eq + Copy + std::hash::Hash> Dp for ModelRpid<T> {
    type State = ScspState;
    type CostType = i32;
    type Label = T;

    fn get_target(&self) -> Self::State {
        ScspState {
            indices: vec![0; self.instance.seqs.len()],
            sol_len: 0,
        }
    }

    fn get_base_cost(&self, state: &Self::State) -> Option<Self::CostType> {
        if state
            .indices
            .iter()
            .zip(self.instance.seqs.iter())
            .all(|(&idx, seq)| idx == seq.len())
        {
            Some(0)
        } else {
            None
        }
    }

    fn get_successors(
        &self,
        state: &Self::State,
    ) -> impl IntoIterator<Item = (Self::State, Self::CostType, Self::Label)> {
        // self.instance
        //     .chars()
        //     .iter()
        //     .filter(|&c| {
        //         state
        //             .indices
        //             .iter()
        //             .zip(self.instance.seqs.iter())
        //             .any(|(&idx, seq)| idx < seq.len() && seq[idx] == *c)
        //     })
        //     .map(|&c| {
        //         let successor = ScspState {
        //             indices: state
        //                 .indices
        //                 .iter()
        //                 .zip(self.instance.seqs.iter())
        //                 .map(|(&idx, seq)| {
        //                     if idx < seq.len() && seq[idx] == c {
        //                         idx + 1
        //                     } else {
        //                         idx
        //                     }
        //                 })
        //                 .collect(),
        //             sol_len: state.sol_len + 1,
        //         };

        //         (successor, 1, c)
        //     })
        //     .collect::<Vec<(Self::State, Self::CostType, Self::Label)>>()

        let mut succs_with_weight = self
            .instance
            .chars()
            .iter()
            .filter(|&c| {
                state
                    .indices
                    .iter()
                    .zip(self.instance.seqs.iter())
                    .any(|(&idx, seq)| idx < seq.len() && seq[idx] == *c)
            })
            .map(|&c| {
                let successor = ScspState {
                    indices: state
                        .indices
                        .iter()
                        .zip(self.instance.seqs.iter())
                        .map(|(&idx, seq)| {
                            if idx < seq.len() && seq[idx] == c {
                                idx + 1
                            } else {
                                idx
                            }
                        })
                        .collect(),
                    sol_len: state.sol_len + 1,
                };
                let weight: usize = state
                    .indices
                    .iter()
                    .zip(self.instance.seqs.iter())
                    .filter(|&(&idx, seq)| idx < seq.len() && seq[idx] == c)
                    .map(|(&idx, seq)| seq.len() - idx)
                    .sum();

                ((successor, 1, c), weight)
            })
            .collect::<Vec<_>>();

        succs_with_weight.sort_by_key(|&(ref _item, weight)| std::cmp::Reverse(weight));
        succs_with_weight
            .into_iter()
            .map(|(item, _weight)| item)
            .collect::<Vec<(Self::State, Self::CostType, Self::Label)>>()
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct ScspKey;

impl<T: Eq + Copy + std::hash::Hash> Dominance for ModelRpid<T> {
    type State = ScspState;
    // type Key = Vec<usize>;
    type Key = ScspKey;

    fn get_key(&self, _state: &Self::State) -> Self::Key {
        // state.indices.clone()
        ScspKey {}
    }

    fn compare(&self, _a: &Self::State, _b: &Self::State) -> Option<std::cmp::Ordering> {
        if _a.sol_len < _b.sol_len {
            if _a
                .indices
                .iter()
                .zip(_b.indices.iter())
                .all(|(&x, &y)| x >= y)
            {
                Some(std::cmp::Ordering::Greater)
            } else {
                None
            }
        } else if _a.sol_len > _b.sol_len {
            if _a
                .indices
                .iter()
                .zip(_b.indices.iter())
                .all(|(&x, &y)| x <= y)
            {
                Some(std::cmp::Ordering::Less)
            } else {
                None
            }
        } else {
            // _a.sol_len == _b.sol_len
            let mut a_is_better = false;
            let mut b_is_better = false;
            for (x, y) in _a.indices.iter().zip(_b.indices.iter()) {
                a_is_better |= x > y;
                b_is_better |= x < y;

                if a_is_better && b_is_better {
                    return None;
                }
            }
            if !a_is_better && !b_is_better {
                Some(std::cmp::Ordering::Equal)
            } else if a_is_better {
                Some(std::cmp::Ordering::Greater)
            } else if b_is_better {
                Some(std::cmp::Ordering::Less)
            } else {
                unreachable!();
            }
        }
    }
}

impl<T: Eq + Copy + std::hash::Hash> Bound for ModelRpid<T> {
    type State = ScspState;
    type CostType = i32;

    fn get_dual_bound(&self, state: &Self::State) -> Option<Self::CostType> {
        let ret = 0;

        // 任意の 2 つの残っている部分の SCS 長の最大値
        let ret = ret.max(
            self.bound_table2
                .iter()
                .zip(state.indices.iter())
                .map(|(bound_row, &i1)| {
                    bound_row
                        .iter()
                        .zip(state.indices.iter())
                        .map(|(dp, &i2)| dp[i1][i2])
                        .max()
                        .unwrap_or(0)
                })
                .max()
                .unwrap_or(0),
        );

        // 任意の 3 つの残っている部分の SCS 長の最大値
        let ret = ret.max(
            self.bound_table3
                .iter()
                .zip(state.indices.iter())
                .map(|(bv1, &i1)| {
                    bv1.iter()
                        .zip(state.indices.iter())
                        .map(|(bv2, &i2)| {
                            bv2.iter()
                                .zip(state.indices.iter())
                                .map(|(bv3, &i3)| bv3[i1][i2][i3])
                                .max()
                                .unwrap_or(0)
                        })
                        .max()
                        .unwrap_or(0)
                })
                .max()
                .unwrap_or(0),
        );

        // 残っている中で長い方から 3 つの SCS 長.
        // let mut first = (0, state.indices[0], self.instance.seqs[0].len());
        // let mut second = (1, state.indices[1], self.instance.seqs[1].len());
        // let mut third = (2, state.indices[2], self.instance.seqs[2].len());
        // if first.2 - first.1 < second.2 - second.1 {
        //     std::mem::swap(&mut first, &mut second);
        // }
        // if second.2 - second.1 < third.2 - third.1 {
        //     std::mem::swap(&mut second, &mut third);
        // }
        // if first.2 - first.1 < second.2 - second.1 {
        //     std::mem::swap(&mut first, &mut second);
        // }
        // for (i, (&idx, seq)) in state.indices.iter().zip(&self.instance.seqs).enumerate() {
        //     if i < 3 {
        //         continue;
        //     }

        //     if seq.len() - idx > first.2 - first.1 {
        //         third = second;
        //         second = first;
        //         first = (i, idx, seq.len());
        //     } else if i != first.0 && seq.len() - idx > second.2 - second.1 {
        //         third = second;
        //         second = (i, idx, seq.len());
        //     } else if i != first.0 && i != second.0 && seq.len() - idx > third.2 - third.1 {
        //         third = (i, idx, seq.len());
        //     }
        // }
        // if first.0 < second.0 {
        //     std::mem::swap(&mut first, &mut second);
        // }
        // if second.0 < third.0 {
        //     std::mem::swap(&mut second, &mut third);
        // }
        // if first.0 < second.0 {
        //     std::mem::swap(&mut first, &mut second);
        // }
        // let ret = self.bound_table3[first.0][second.0][third.0][first.1][second.1][third.1];

        Some(ret)
    }
}
