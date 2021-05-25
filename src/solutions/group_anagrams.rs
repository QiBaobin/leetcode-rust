// https://leetcode.com/problems/group-anagrams/

struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams: Vec<Vec<String>> = Vec::with_capacity(strs.len());
        let mut sets: Vec<HashMap<char, usize>> = Vec::with_capacity(strs.len());
        use std::collections::HashMap;
        let mut s_set = HashMap::new();
        for s in strs {
            for c in s.chars() {
                *s_set.entry(c).or_insert(0) += 1;
            }
            if let Some((i, _)) = sets.iter().enumerate().find(|&(i, ss)| ss.eq(&s_set)) {
                anagrams[i].push(s);
                s_set.clear();
            } else {
                sets.push(s_set);
                anagrams.push(vec![s]);
                s_set = HashMap::new();
            }
        }
        anagrams
    }
}
