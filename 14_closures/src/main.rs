use colored::*;
use std::thread;

#[derive(Debug, Clone)]
pub struct City {
    pub name: String,
    pub population: i64,
    pub country: String,
}

fn sort_cities(cities: &mut  Vec<City>) {
    cities.sort_by_key(|city| -city.population);
}


fn start_sorting_thread(mut cities: Vec<City>) -> thread::JoinHandle<Vec<City>> {
    let key_fn = move |city: &City| -> i64 { -city.population };
    let handle = thread::spawn(move || {
        cities.sort_by_key(key_fn);
        cities
    });
    handle
}

fn call_twice<F>(mut closure: F) where F: FnMut() {
    closure();
    closure();
}

fn main() {
    let cities = vec![
        City {
            name: "Dublin".to_string(),
            population: 1_000_000,
            country: "Ireland".to_string(),
        },
        City {
            name: "Belfast".to_string(),
            population: 500_000,
            country: "Ireland".to_string(),
        },
        City {
            name: "London".to_string(),
            population: 10_000_000,
            country: "England".to_string(),
        },
    ];
    println!("Cities: {:?}", cities);
    println!("{}", "--- Closures That BORROW ---".green());
    let mut new_cities = cities.clone();
    sort_cities(&mut new_cities);
    println!("Sorted Cities: {:?}", new_cities);

    println!("{}", "--- Closures That MUT BORROW ---".green());
    let mut new_cities = cities.clone();
    let update_population = || {
        for city in &mut new_cities {
            city.population /= 100;
        }
    };
    call_twice(update_population);
    println!("Updated Cities: {:?}", new_cities);

    println!("{}", "--- Closures That STEAL ---".green());
    let new_cities = cities.clone();
    let handle = start_sorting_thread(new_cities);
    println!("{}", "new_cities have been consumed".yellow());
    println!("Sorted Cities: {:?}", handle.join().unwrap());
}
