extern crate rand;

mod cuckoo_hash_map;
mod hashes;

use cuckoo_hash_map::CuckooHashTable;
use hashes::{random_table_input};
use hashes::{two_indep_hash_family, three_indep_hash_family, five_indep_hash_family};

fn correctness_tests(size: usize) {
	println!("Correctness Tests for single-threaded version:");
	let hashes = [two_indep_hash_family, three_indep_hash_family, five_indep_hash_family];
	for hash in hashes.iter() {
		let mut table: CuckooHashTable = CuckooHashTable::new(size, hash());
		table.insert(5);
		table.insert(7);
		assert!(table.contains(5));
		assert!(table.contains(7));
		assert!(!table.contains(42));
		assert!(!table.contains(137));
		table.remove(5);
		assert!(!table.contains(5));
		table.remove(7);
		assert!(!table.contains(7));
		table.insert(5);
		table.insert(5);
		assert!(table.contains(5));
	}
	println!("  Pass!");
}

fn stress_test(table_size: usize, input_size: usize) {
	println!("Stress Test for single-threaded version:");
	let mut table: CuckooHashTable = CuckooHashTable::new(table_size, five_indep_hash_family());
	let mut inputs = Vec::with_capacity(input_size);
	for _ in 0..input_size {
		inputs.push(random_table_input());
	}
	for input in inputs.clone() {
		table.insert(input);
	}
	for input in inputs.clone() {
		assert!(table.contains(input));
	}
	for input in inputs.clone() {
		table.remove(input);
	}
	for input in inputs {
		assert!(!table.contains(input));
	}
	println!("  No correctness errors!");
}

fn main() {
	correctness_tests(1024);
	stress_test(1 << 22, 1 << 20);
  println!("All done!");
}
