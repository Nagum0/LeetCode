use leetlib::arrays_and_hashing::*;

fn main() {
	let nums = vec![1,1,1,2,2,3];
	let k = 2;
	println!("{:?}", top_k_frequent(nums, k));

	let nums = vec![1];
	let k = 1;
	println!("{:?}", top_k_frequent(nums, k));
}