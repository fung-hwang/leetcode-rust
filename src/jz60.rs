struct Solution;

impl Solution {
    pub fn dices_probability(n: i32) -> Vec<f64> {
        use std::cmp::{max, min};
        // let n = n as usize;
        let mut probability = vec![1.0 / 6.0; 7];
        for nth in 2..=n {
            let mut tmp = vec![0.0; (nth * 6) as usize + 1];
            for i in nth..=(nth * 6) {
                tmp[i as usize] = (max(nth - 1, i - 6)..=(min(i - 1, (nth - 1) * 6)))
                    .map(|k| probability[k as usize])
                    .sum::<f64>()
                    / 6.0;
            }
            probability = tmp;
        }

        (&probability[n as usize..]).to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{:?}", Solution::dices_probability(2));
    }
}
