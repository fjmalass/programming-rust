use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AllocType {
    VecWithCapacity,
    VecNew,
    Array,
}



#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Config {
    pub alloc_type: AllocType,
    pub iterations: usize,
    pub size: usize,
    pub elapsed: Duration,
}

///
/// Compare the performance of allocating a vector with capacity vs a vector without capacity and an array
/// The results are as follows: hardly any difference between the two vectors, but the array is
/// much faster (from seconds to nano seconds)
fn main() {
    let iterations_vec: Vec<usize> = vec![10, 100, 500, 1_000];
    let size: usize = 10_000_000;

    let mut vect_array_timing: Vec<Config> = Vec::with_capacity(iterations_vec.len());
    {
        for iterations in &iterations_vec {
            let start = Instant::now();
            let mut v: [usize; 10_000_000] = [0; 10_000_000];
            for _ in 0..*iterations {
                for i in 0..size {
                    v[i] = i;
                }
            }
            let elapsed= start.elapsed();
            let config = Config {
                alloc_type: AllocType::Array,
                iterations: *iterations,
                size,
                elapsed,
            };
            vect_array_timing.push(config);
        }
    }
    println!("Array: {:#?}", vect_array_timing);

    let mut vect_capacity_timing: Vec<Config> = Vec::with_capacity(iterations_vec.len());
    {
        for iterations in &iterations_vec {
            let start = Instant::now();
            for _ in 0..*iterations {
                let mut v: Vec<usize> = Vec::with_capacity(size);
                for i in 0..size {
                    v.push(i);
                }
            }
            let config = Config {
                alloc_type: AllocType::VecWithCapacity,
                iterations: *iterations,
                size,
                elapsed: start.elapsed(),
            };
            vect_capacity_timing.push(config);
        }
    }
    println!("Vector With Capacity: {:#?}", vect_capacity_timing);

    let mut vect_new_timing: Vec<Config> = Vec::with_capacity(iterations_vec.len());
    {
        for iterations in &iterations_vec {
            let start = Instant::now();
            for _ in 0..*iterations {
                let mut v: Vec<usize> = Vec::new();
                for i in 0..size {
                    v.push(i);
                }
            }
            let config = Config {
                alloc_type: AllocType::VecNew,
                iterations: *iterations,
                size,
                elapsed: start.elapsed(),
            };
            vect_new_timing.push(config);
        }
    }
    println!("Vector New: {:#?}", vect_new_timing);

}
