// Cuckoo Hash Table

use std::f32;
use std::sync::Arc;

use hashes::HashFunction;
use hashes::HashFamily;

const NUM_TABLES: usize = 2;

struct CuckooHashEntry {
	value: i32,
	occupied: bool
}

pub struct CuckooHashTable {
	size: usize,
	buckets_per_table: usize,
	total_buckets: usize,
	buckets: Vec<Vec<CuckooHashEntry>>,
	hashes: Vec<Arc<HashFunction>>,
	family: Arc<HashFamily>
}

impl CuckooHashTable {

	pub fn new(num_buckets: usize, family: Arc<HashFamily>) -> CuckooHashTable {
		let table_size = num_buckets / NUM_TABLES;
		let mut t = Vec::with_capacity(NUM_TABLES);
		for i in 0..NUM_TABLES {
			t.push(Vec::with_capacity(table_size));
			for _ in 0..table_size {
				t[i].push(CuckooHashEntry {
					value: 0,
					occupied: false
				})
			}
		}
		let mut hashes = Vec::new();
		for _ in 0..NUM_TABLES {
			hashes.push((family.get)());
		}

		CuckooHashTable {
			size: 0,
			total_buckets: num_buckets,
			buckets_per_table: num_buckets / NUM_TABLES,
			buckets: t,
			hashes: hashes,
			family: family
		}
	}

	pub fn insert(&mut self, key: i32) {
		let mut data = key;
		if self.contains(data) { return; }
		if self.size == self.total_buckets { panic!("Table capacity reached"); }

		let max_displacements: usize = (6.0 * f32::log2((self.size as f32) + 1.0) + 1.0) as usize;
		for d in 0..max_displacements {
			let table: usize = d % NUM_TABLES;
			let hash: usize = self.get_bucket(data, table);
			if self.buckets[table][hash].occupied == false {
				self.buckets[table][hash].occupied = true;
				self.buckets[table][hash].value = data;
				self.size += 1;
				return;
			} else {
				let displaced: i32 = self.buckets[table][hash].value;
				self.buckets[table][hash].value = data;
				data = displaced;
			}
		}
		self.rehash(data);
		self.size += 1;
	}

	pub fn contains(&self, key: i32) -> bool {
		let mut index = Vec::new();
		for i in 0..NUM_TABLES {
			index.push(self.get_bucket(key, i));
		}
		for i in 0..NUM_TABLES {
			if self.buckets[i][index[i]].occupied {
				if self.buckets[i][index[i]].value == key {
					return true;
				}
			}
		}
		return false;
	}

	pub fn remove(&mut self, key: i32) {
		let mut index = Vec::new();
		for i in 0..NUM_TABLES {
			index.push(self.get_bucket(key, i));
		}
		for i in 0..NUM_TABLES {
			if self.buckets[i][index[i]].occupied {
				if self.buckets[i][index[i]].value == key {
					self.buckets[i][index[i]].occupied = false;
					self.size -= 1;
					return;
				}
			}
		}
	}

	fn get_bucket(&self, key: i32, table: usize) -> usize {
		return self.hashes[table](key) % self.buckets_per_table;
	}

	fn rehash(&mut self, key: i32) {
		// Store all the values in the table--we'll need a stable vector of values for later.
		let mut values = Vec::new();
		values.push(key);
		for i in 0..NUM_TABLES {
			for j in 0..self.buckets_per_table {
				if self.buckets[i][j].occupied { values.push(self.buckets[i][j].value); }
			}
		}
		// Keep going until a configuration is found that works.
		loop {
			// Clear all the tables so we can try reinserting them with new hash functions.
			for i in 0..NUM_TABLES {
				for j in 0..self.buckets_per_table {
					self.buckets[i][j].occupied = false;
				}
			}
			// Generate new hash functions from our family.
			for i in 0..NUM_TABLES {
				self.hashes[i] = (self.family.get)();
			}
			//Repeat the insert procedure until either rehash is successful or, if we fail, continue while loop.
			for value in values {
				let mut data = value;
				let max_displacements: usize = (6.0 * f32::log2((self.size as f32) + 1.0) + 1.0) as usize;
				for d in 0..max_displacements {
					let table: usize = d % NUM_TABLES;
					let hash: usize = self.get_bucket(data, table);
					if self.buckets[table][hash].occupied == false {
						self.buckets[table][hash].occupied = true;
						self.buckets[table][hash].value = data;
						continue;
					} else {
						let displaced: i32 = self.buckets[table][hash].value;
						self.buckets[table][hash].value = data;
						data = displaced;
					}
				}
				// If we fail to insert in under 6 lg n displacements, restart the rehashing procedure.
				continue;
			}
			// If all insertions are successful, rehash is successful, so break out of while loop.
			break;
		}
	}
}