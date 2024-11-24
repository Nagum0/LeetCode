use leetlib::arrays_and_hashing::*;

fn main() {
	let nums: Vec<i32> = vec![100,4,200,1,3,2];
	println!("{}", longest_consecutive(nums));
	
	let nums: Vec<i32> = vec![0,3,7,2,5,8,4,6,0,1];
	println!("{}", longest_consecutive(nums));
}