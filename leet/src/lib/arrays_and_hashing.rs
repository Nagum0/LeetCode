#![allow(unused)]

use std::{collections::{HashMap, HashSet}, hash::Hash};

/// # Contains Duplicate - Easy
/// 
/// Input: nums = [1,2,3,1]
/// Output: true
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
	let mut num_set: HashSet<i32> = HashSet::new();

	for i in nums {
		if num_set.contains(&i) {
			return true;
		}
		else {
			num_set.insert(i);
		}
	}

	false
}

/// # Valid Anagram - Easy
/// 
/// Input: s = "anagram", t = "nagaram"
/// Output: true
/// 
/// Input: s = "rat", t = "car"
/// Output: false
pub fn is_anagram(s: String, t: String) -> bool {
	let mut s_char_count_map: HashMap<char, usize> = HashMap::new();
	let mut t_char_count_map: HashMap<char, usize> = HashMap::new();

	if s.len() != t.len() {
		return false;
	}

	for (cs, ct) in s.chars().zip(t.chars()) {
		s_char_count_map
			.entry(cs)
			.and_modify(|v| *v += 1)
			.or_insert(1);
		t_char_count_map
			.entry(ct)
			.and_modify(|v| *v += 1)
			.or_insert(1);
	}

	s_char_count_map == t_char_count_map
}

/// # Two Sum - Easy
/// 
/// Input: nums = [2,7,11,15], target = 9
/// Output: [0,1]
/// 
/// Input: nums = [3,2,4], target = 6
/// Output: [1,2]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
	let mut map: HashMap<i32, usize> = HashMap::new();

	for (i, n) in nums.iter().enumerate() {
		let diff = target - *n;

		if map.contains_key(&(target - diff)) {
			return vec![*map.get(&(target - diff)).unwrap() as i32, i as i32];
		}
		else {
			map.insert(diff, i);
		}
	}

	vec![]
}

/// # Group Anagrams - Medium
/// 
/// Input: strs = ["eat","tea","tan","ate","nat","bat"]
/// Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
/// 
/// Input: strs = ["tea","tea","tea"]
/// Output [["tea","tea","tea"]]
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
	let mut map: HashMap<[usize; 26], Vec<String>> = HashMap::new();

	for str in strs {
		let mut char_array: [usize; 26] = [0; 26];

		for c in str.chars() {
			char_array[c as usize - 'a' as usize] += 1;
		}

		map
			.entry(char_array)
			.and_modify(|vec| vec.push(str.clone()))
			.or_insert(vec![str]);
	}

	map
		.values()
		.fold(Vec::new(), |mut acc, v| {
		acc.push(v.to_vec());
		acc
	})
}

/// # Top K Frequent Elements - Medium
/// 
/// Input: nums = [1,1,1,2,2,3], k = 2
/// Output: [1,2]
/// 
/// Input: nums = [1], k = 1
/// Output: [1]
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
	let mut map: HashMap<i32, i32> = HashMap::new();

	// Count the frequency of the elements
	for i in nums {
		map
			.entry(i)
			.and_modify(|v| *v += 1)
			.or_insert(1);
	}

	// Turn map into a Vec<(i32, usize)>
	let mut vals: Vec<(i32, i32)> = map.iter().map(|(k, v)| (*k, *v)).collect();

	// Sort vals by descending order
	vals.sort_by(|a, b| b.1.cmp(&a.1));

	// Return the the first k elements
	let mut count: i32 = 0;
	vals.iter().fold(Vec::new(), |mut acc, i| {
		if count < k {
			acc.push(i.0);
			count += 1;
		}
		acc
	})
}