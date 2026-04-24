use std::collections::HashSet;
use std::hash::Hash;

#[derive(Clone)]
pub struct ScspInstance<T: Eq + Copy + Hash> {
    pub seqs: Vec<Vec<T>>,
    set: HashSet<T>,
}

impl<T: Eq + Copy + Hash> ScspInstance<T> {
    pub fn new(seqs: &[Vec<T>]) -> Self {
        let mut seqs_vec = Vec::new();
        let mut set: HashSet<T> = HashSet::new();
        for seq in seqs {
            let mut seq_vec = Vec::new();
            for &c in seq {
                set.insert(c);
                seq_vec.push(c);
            }
            seqs_vec.push(seq_vec);
        }

        Self {
            seqs: seqs_vec,
            set,
        }
    }

    /// 問題インスタンスに出現する文字の集合を返す.
    pub fn set(&self) -> HashSet<T> {
        self.set.clone()
    }
}

impl std::str::FromStr for ScspInstance<char> {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            &s.lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .map(|line| line.chars().collect())
                .collect::<Vec<Vec<char>>>(),
        ))
    }
}

#[derive(Default)]
pub struct ScspSolution<T: Eq + Copy + std::hash::Hash> {
    pub seq: Option<Vec<T>>,
    pub bound: Option<i32>,
}

impl<T: Eq + Copy + std::hash::Hash> ScspSolution<T> {
    /// 解の目的関数値を返す.
    pub fn objective(&self) -> Option<i32> {
        self.seq.as_ref().map(|x| x.len() as i32)
    }

    /// 解が実行可能かどうか, つまり問題インスタンスの各文字列の超配列になっているかどうかを判定する.
    pub fn is_feasible(&self, instance: &ScspInstance<T>) -> bool {
        let Some(sol) = &self.seq else {
            return false;
        };

        for seq in &instance.seqs {
            let mut state = 0usize;
            for &c in sol {
                if state >= seq.len() {
                    break;
                } else if c == seq[state] {
                    state += 1;
                }
            }

            if state < seq.len() {
                return false;
            }
        }

        true
    }

    /// 解が最適かどうか判定する.
    /// 実行可能解が与えられておりかつそのコストがバウンドと等しいときかつそのときに限り最適である.
    pub fn is_optimal(&self, instance: &ScspInstance<T>) -> bool {
        let Some(obj) = self.objective() else {
            return false;
        };
        let Some(bound) = self.bound else {
            return false;
        };

        self.is_feasible(instance) && obj == bound
    }
}
