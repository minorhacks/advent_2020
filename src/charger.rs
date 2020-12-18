use std::collections::HashMap;
use std::collections::VecDeque;

fn calc_deltas(charger_list: &[i32]) -> Vec<i32> {
    let mut charger_list = Vec::from(charger_list);
    charger_list.push(0); // Wall is 0 jolts, implicitly
    charger_list.sort_unstable();
    charger_list.push(charger_list[charger_list.len() - 1] + 3); // Device is max + 3 jolts, implicitly. Max is now the last element after sorting.

    let mut deltas = Vec::new();
    for i in 0..charger_list.len() - 1 {
        deltas.push(charger_list[i + 1] - charger_list[i]);
    }
    deltas
}

pub fn difference_distribution(charger_list: &[i32]) -> HashMap<i32, usize> {
    let deltas = calc_deltas(charger_list);
    deltas.into_iter().fold(HashMap::new(), |mut acc, delta| {
        *acc.entry(delta).or_insert(0) += 1;
        acc
    })
}

fn tribonacci_lookup(len: usize) -> HashMap<usize, i64> {
    let mut lookup = HashMap::new();
    lookup.insert(1, 1);
    lookup.insert(2, 2);
    lookup.insert(3, 4);
    for i in 4..=len {
        lookup.insert(i, lookup[&(i - 3)] + lookup[&(i - 2)] + lookup[&(i - 1)]);
    }
    lookup
}

// So, this function is fuckin' sneaky. The problem statement says that chargers
// can be 1, 2, or 3 jolts from the previous. However, in both sample inputs and
// problem input, only deltas of 3's and 1's seem to be present - no 2's!
//
// This function works based off of this invariant: the idea here is that runs
// of 3's mean no variants are possible (can't remove a charger because then
// there will be a gap of 6). Runs of 1's create variants, with a specific
// pattern - a run of n 1's creates tribonacci(n) variants, where tribonacci is
// in the sequence of tribonacci numbers (starting a little offset: 1, 2, 4, 7,
// 13, etc.)
//
// Multiplying the number of variants caused by each run of 1's gets you the
// total number of variants.
pub fn valid_combinations_count(charger_list: &[i32]) -> i64 {
    let lookup = tribonacci_lookup(50);
    calc_deltas(charger_list)
        .into_iter()
        .map(|delta| delta.to_string())
        .collect::<Vec<_>>()
        .join("")
        .split('3')
        .filter(|s| !s.is_empty())
        .map(|ones_str| lookup.get(&ones_str.len()).unwrap())
        .product()
}

// for posterity
#[allow(dead_code)]
fn valid_combinations_count_naive(charger_list: &[i32]) -> usize {
    let mut charger_list = Vec::from(charger_list);
    charger_list.sort_unstable();
    let &max = charger_list.iter().max().unwrap();
    let mut combos = VecDeque::new();
    let mut combos_count = 0;
    combos.push_back(vec![0]);
    while !combos.is_empty() {
        let combo = combos.pop_front().unwrap();
        for i in 1..=3 {
            let current_max = combo[combo.len() - 1];
            if charger_list.contains(&(current_max + i)) {
                if max == current_max + i {
                    combos_count += 1;
                } else {
                    let mut combo = combo.clone();
                    combo.push(current_max + i);
                    combos.push_back(combo);
                }
            }
        }
    }
    combos_count
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &[i32] = &[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

    static TEST_INPUT_2: &[i32] = &[
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];

    #[test]
    fn test_difference_distribution_simple() {
        let distribution = difference_distribution(TEST_INPUT_1);
        println!("{:?}", distribution);
        assert_eq!(7, distribution[&1]);
        assert_eq!(5, distribution[&3]);
    }

    #[test]
    fn test_difference_distribution_long() {
        let distribution = difference_distribution(TEST_INPUT_2);
        println!("{:?}", distribution);
        assert_eq!(22, distribution[&1]);
        assert_eq!(10, distribution[&3]);
    }

    #[test]
    fn test_valid_combinations_count() {
        assert_eq!(8, valid_combinations_count(TEST_INPUT_1));
        assert_eq!(19208, valid_combinations_count(TEST_INPUT_2));

        assert_eq!(1, valid_combinations_count(&[3, 4, 7])); // 3 1 3
        assert_eq!(2, valid_combinations_count(&[3, 4, 5, 8])); // 3 1 1 3
        assert_eq!(4, valid_combinations_count(&[3, 4, 5, 6, 9])); // 3 1 1 1 3
        assert_eq!(7, valid_combinations_count(&[3, 4, 5, 6, 7, 10])); // 3 1 1 1 1 3
        assert_eq!(13, valid_combinations_count(&[3, 4, 5, 6, 7, 8, 11])); // 3 1 1 1 1 1 3
        assert_eq!(24, valid_combinations_count(&[3, 4, 5, 6, 7, 8, 9, 12]));
        // 3 1 1 1 1 1 1 3
    }
}
