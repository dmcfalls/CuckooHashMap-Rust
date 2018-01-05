extern crate rand;

use std::boxed::Box;
use std::sync::Arc;
use self::rand::distributions::IndependentSample;

const K_LARGE_PRIME: usize = (1 << 31) - 1;

pub type HashFunction = Fn(i32) -> usize;

#[derive(Debug, Clone)]
pub struct HashFamily {
	pub get: Box<fn() -> Arc<HashFunction>>,
	pub name: String
}

pub fn random_table_input() -> i32 {
	let between = rand::distributions::Range::new(0, 1 << 30);
	let mut rng = rand::thread_rng();
	return between.ind_sample(&mut rng);
}

pub fn random_field_elem() -> usize {
	let between = rand::distributions::Range::new(0, K_LARGE_PRIME - 1);
	let mut rng = rand::thread_rng();
	return between.ind_sample(&mut rng);
}

pub fn two_indep_hash_family() -> Arc<HashFamily> {
	fn get_implementation() -> Arc<HashFunction> {
		let a: usize = random_field_elem();
		let b: usize = random_field_elem();
		return Arc::new(move |key: i32| {
			return (a * (key as usize) + b) % K_LARGE_PRIME;
		});
	}

	Arc::new(HashFamily {
		get: Box::new(get_implementation),
		name: "2-Independent Hash Family".into()
	})
}

pub fn three_indep_hash_family() -> Arc<HashFamily> {
	fn get_implementation() -> Arc<HashFunction> {
		let a: usize = random_field_elem();
		let b: usize = random_field_elem();
		let c: usize = random_field_elem();
		return Arc::new(move |key: i32| {
			return (a * (key as usize) * (key as usize) + b * (key as usize) + c) % K_LARGE_PRIME;
		});
	}

	Arc::new(HashFamily {
		get: Box::new(get_implementation),
		name: "3-Independent Hash Family".into()
	})
}

pub fn five_indep_hash_family() -> Arc<HashFamily> {
	fn get_implementation() -> Arc<HashFunction> {
		let a: usize = random_field_elem();
		let b: usize = random_field_elem();
		let c: usize = random_field_elem();
		let d: usize = random_field_elem();
		let e: usize = random_field_elem();
		return Arc::new(move |key: i32| {
			let ukey: usize = key as usize;
			return (a * ukey * ukey * ukey * ukey + b * ukey * ukey * ukey + c * ukey * ukey + d * ukey + e) % K_LARGE_PRIME;
		});
	}

	Arc::new(HashFamily {
		get: Box::new(get_implementation),
		name: "5-Independent Hash Family".into()
	})
}