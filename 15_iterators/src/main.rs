use colored::*;
use std::iter::successors;
use std::time::Instant;

pub fn fibonacci_mut(state: &mut (u64, u64)) -> impl Iterator<Item = u64> + '_ {
    std::iter::from_fn(move || {
        let next = state.0 + state.1;
        *state = (state.1, next);
        Some(next)
    })
}

pub fn fibonacci() -> impl Iterator<Item = u64> {
    let mut state = (0, 1);
    std::iter::from_fn(move || {
        let next = state.0 + state.1;
        state = (state.1, next);
        Some(next)
    })
}

fn main() {
    let mut a = [0 as i64; 10_000];
    for i in 0..a.len() {
        a[i] = i as i64;
    }
    {
        println!("{}", "-- For loop -- ".green().bold());
        let start = Instant::now();
        let mut sum = 0;
        for i in 0..10_000 {
            sum += a[i];
        }
        println!("Sum: {}, in {} μs", sum, start.elapsed().as_micros());

        println!("{}", "-- Iterator --".green().bold());
        let start = Instant::now();
        let sum = (0..10_000).fold(0, |sum, i| sum + a[i]);
        println!("Sum: {}, in {} μs", sum, start.elapsed().as_micros());

        println!("{}", "-- Into Iterator with Array --".green().bold());
        // let v: Vec<i32> = (0..10_000).collect();
        let start = Instant::now();
        let mut sum = 0;
        for i in &a {
            sum += i;
        }
        println!("Sum: {}, in {} μs", sum, start.elapsed().as_micros());

        // Equivalent
        let mut iterator = (&a).into_iter();
        let start = Instant::now();
        let mut sum = 0;
        while let Some(i) = iterator.next() {
            sum += i;
        }
        println!(
            "Explicit into Sum: {}, in {} us",
            sum,
            start.elapsed().as_micros()
        );
    }

    println!("{}", "-- HEAP HEAP -- ".green().bold());
    let v: Vec<i64> = (0..10_000).collect();
    {
        println!("{}", "-- For loop -- ".green().bold());
        let start = Instant::now();
        let mut sum = 0;
        for i in 0..v.len() {
            sum += v[i];
        }
        println!("Sum: {}, in {} μs", sum, start.elapsed().as_micros());

        println!("{}", "-- Iterator --".green().bold());
        let start = Instant::now();
        let sum = (0..v.len()).fold(0, |sum, i| sum + v[i]);
        println!("Sum: {}, in {} μs", sum, start.elapsed().as_micros());

        println!("{}", "-- Into Iterator with Vec --".green().bold());
        // let v: Vec<i32> = (0..10_000).collect();
        let start = Instant::now();
        let mut sum = 0;
        for i in &v {
            sum += i;
        }
        println!("Sum: {}, in {} μs", sum, start.elapsed().as_micros());

        // Equivalent
        let mut iterator = (&v).into_iter();
        let start = Instant::now();
        let mut sum = 0;
        while let Some(i) = iterator.next() {
            sum += i;
        }
        println!(
            "Explicit into Sum: {}, in {} μs",
            sum,
            start.elapsed().as_micros()
        );
    }

    // Successors
    println!("{}", "-- SUCCESSORS --".green().bold());
    let start = Instant::now();
    let _v = successors(Some(0), |i| Some(i + 1))
        .take(10_000)
        .collect::<Vec<i64>>();
    println!(
        "Vector SUCCESSOR generated in {} μs",
        start.elapsed().as_micros()
    );

    let start = Instant::now();
    let _v: Vec<i64> = (0..10_000).collect();
    println!(
        "Vector COLLECT generated in {} μs",
        start.elapsed().as_micros()
    );

    let start = Instant::now();
    let mut a = [0 as i64; 10_000];
    for i in 0..a.len() {
        a[i] = i as i64;
    }
    println!(
        "Vector ARRAY  generated in {} μs",
        start.elapsed().as_micros()
    );

    // from_fn
    println!("{}", "-- FROM_FN --".green().bold());
    let start = Instant::now();
    let f = fibonacci().take(10).collect::<Vec<_>>();
    println!("fibonnaci:     {:?} in {:?} us", f, start.elapsed());

    let start = Instant::now();
    let mut state = (0, 1);
    let f = fibonacci_mut(&mut state).take(10).collect::<Vec<_>>();
    println!("fibonacci_mut: {:?}, state: {:?}, in {} us", f, state, start.elapsed().as_micros());

    // DRAIN
    println!("{}", "-- DRAIN --".green().bold());
    let mut outer = "Earth".to_string();
    let inner = String::from_iter(outer.drain(1..4));
    println!("outer: {}, inner: {}", outer, inner);

    // Map FIlter
    println!("{}", "-- MAP FILTER --".green().bold());
    println!(
        "{}",
        "we need to dereference the `&&s` argument in the filter".yellow()
    );
    let text = "   plenty\n of    \n  spaces  \n everywhere".to_string();
    let start = Instant::now();
    let v = text
        .lines()
        .map(str::trim)
        .filter(|s| *s != "of") // argument is in facet &&s so we need to dereference it;
        .collect::<Vec<_>>();
    println!("v: {:?}, in {:?} ns", v, start.elapsed().as_nanos());

    // for loop
    let mut v: Vec<&str> = vec![];
    for line in text.lines() {
        let line = line.trim();
        if line != "of" {
            v.push(line)
        }
    }
    println!(
        "For loop v: {:?}, in {:?} ns",
        v,
        start.elapsed().as_nanos()
    );
}
