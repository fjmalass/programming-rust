use colored::*;
use std::collections::BTreeMap;
use std::iter::successors;
use std::str::FromStr;
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

// Manual Iterator
#[derive(Debug)]
struct MyRange {
    start: usize,
    end: usize,
}

impl Iterator for MyRange {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.start >= self.end {
            None
        } else {
            let result = Some(self.start);
            self.start += 1;
            result
        }
    }
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
    println!(
        "fibonacci_mut: {:?}, state: {:?}, in {} us",
        f,
        state,
        start.elapsed().as_micros()
    );

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

    // filter_map
    println!("{}", "-- FILTER  MAP --".green().bold());
    println!(
        "{}",
        "Using mapping to determine what needs to be filtered".yellow()
    );
    let text = "1\nfrond .25 0289\n 3.141231 est\n".to_string();

    // use ok() converts a result to an option
    let start = Instant::now();
    let number_it = text
        .split_whitespace()
        .filter_map(|s| f64::from_str(s).ok());
    let numbers = number_it.collect::<Vec<_>>();
    println!(
        "numbers with filter_map: {:?} in {} ns",
        numbers,
        start.elapsed().as_nanos()
    );
    //
    // Naive and long way to do it but similar timing as the difference is based on when you call
    // the function
    let start = Instant::now();
    let number_it = text
        .split_whitespace()
        .map(|s| f64::from_str(s))
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap());

    let numbers = number_it.collect::<Vec<_>>();
    println!(
        "numbers with separate:   {:?} in {} ns",
        numbers,
        start.elapsed().as_nanos()
    );

    //  flat_map
    println!("{}", "-- FLAT  MAP --".green().bold());
    let mut parks = BTreeMap::new();
    parks.insert("Portland", vec!["Mt. Tabor Park", "Forest Park"]);
    parks.insert("Kyoto", vec!["Tadasu-o-Mort Forest", "Maruyama Koen"]);
    parks.insert("Raleigh", vec!["Umpstead Park", "Lake Johnson Preserve"]);
    let all_parks = parks.values().flatten().collect::<Vec<_>>();
    println!("all_parks: {}: {:?}", all_parks.len(), all_parks);

    let all_parks_caps = all_parks
        .iter()
        .map(|s| s.to_uppercase())
        .collect::<Vec<_>>();
    println!("all_parks caps  {:?}, ", all_parks_caps);

    // Iterator BY_REF
    println!("{}", "-- BY_REF --".green().bold());
    println!(
        "{}",
        "To be able to use the iterator and resume at the position it left off".yellow()
    );
    let message = "To: jimb\r\n\
                   From: superego <junk@gmail.com>\r\n\
                   \r\n\
                   Oooh this is the body\r\n\
                   Get some donuts on the way home.\r\n";
    let mut lines = message.lines();
    let headers_iter = lines.by_ref().take_while(|line| !line.is_empty());
    let headers = headers_iter.collect::<Vec<_>>();
    println!("headers: {:?}", headers);
    let body = lines.collect::<Vec<_>>();
    println!("body: {:?}", body);

    // Cycle
    println!("{}", "-- CYCLE --".green().bold());
    let colors = ["red", "green", "blue"].iter().cycle();
    let now_colors = colors.take(10).collect::<Vec<_>>();
    println!("colors: {:?}", now_colors);

    // Position
    println!("{}", "-- POSITION --".green().bold());
    let text = "Xerxes".to_string();
    let mut text_iter = text.chars();
    let p = text_iter.position(|c| c == 'e');
    println!("p for 'e': {:?}", p);
    let p = text_iter.position(|c| c == 'e');
    println!("p for the next 'e' (relative): {:?}", p);
    let p = text_iter.position(|c| c == 'e');
    println!("p for the next 'e' (relative): {:?}", p);

    // fold
    println!("{}", "-- FOLD --".green().bold());
    println!(
        "{}",
        "fold accumulates values, used for sums, joins etc. ".yellow()
    );
    let a = ["one", "two", "three"];
    let s = a.iter().fold(String::new(), |acc, &s| acc + s + ", ");
    println!("s: {}", s);

    let a = [1, 2, 3];
    let sum = a.iter().fold(0, |acc, &n| acc + n);
    println!("sum: {:?} = {}", a, sum);

    // Manual iterators
    println!("{}", "-- Manually Generated Iterators --".green().bold());
    let mut m_iter = MyRange { start: 0, end: 10 };
    println!("m_iter: {:?}", m_iter);
    println!("m_iter.next(): {:?}, {:?}", m_iter.next(), m_iter);
    m_iter.start = 11;
    println!("m_iter.next(): {:?}, {:?}", m_iter.next(), m_iter);
}
