use colored::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::time::Instant;

use ahash::AHashMap;
use fnv::FnvHashMap;

use std::str::FromStr;

use std::borrow::Cow;
use std::fmt::Write;
use std::net::IpAddr;

// Hash
//pub type FnvHashMap<K, V> = HashMap<K, V, FnHashBuilder>;

pub fn less_than_nan<T>(a: T, b: T) -> std::cmp::Ordering
where
    T: PartialOrd + Copy + std::cmp::PartialOrd,
{
    match a.partial_cmp(&b) {
        Some(ord) => ord,
        None if a != a /* not a number */ => std::cmp::Ordering::Less,
        _ => std::cmp::Ordering::Greater,
    }
}

// returns a copied  ersion of the static value from env, if not,  will create a new string
fn get_name() -> Cow<'static, str> {
    match std::env::var("NAME") {
        Ok(name) => Cow::Owned(name),
        Err(_) => Cow::Borrowed("stranger"),
    }
    // or
    // std::env::var("NAME")
    //     .map(Cow::Owned)
    //     .unwrap_or(Cow::Borrowed("stranger"))
}
fn get_name_using_into() -> Cow<'static, str> {
    std::env::var("NAME")
        .map(|v| v.into())
        .unwrap_or("stranger".into())
}

fn main() {
    println!(
        "{}",
        "+----------------+-----------------------
| Rust           | C++
+----------------+-----------------------
| Vec<T>         | std::vector
| VecDeque<T>    | std::deque
| LinkedList<T>  | std::list
| BinaryHeap<T>  | std::priority_queue
| HashMap<K, V>  | std::unordered_map
| BTreeMap<K, V> | std::map
| HashSet<T>     | std::unordered_set
| BTreeSet<T>    | std::set
+----------------+-----------------------"
            .yellow()
    );

    println!("{:-^20}", " VEC ".green().bold());

    let v = vec![7u32; 8];
    println!("v: {:?}", v);

    let temps = vec![12.1, 23.5, 8.1, 10.0];
    let changes = temps.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    println!("temps: {:?}, changes: {:?}", temps, changes);

    // Sorting float and handling nan example
    let vf = vec![1., 2., -3., 4., 5., f32::NAN];
    println!(
        "{}",
        "Sorting a) stripping Nan and b) handling NaN".yellow()
    );

    let mut vf_cloned = vf.clone();
    let start_with_nan = Instant::now();
    vf_cloned.sort_by(|a, b| less_than_nan(a, b));
    let elapsed_with_nan = start_with_nan.elapsed().as_nanos();

    let start_stripped = Instant::now();
    let mut vf_stripped = vf.iter().filter(|x| !x.is_nan()).collect::<Vec<_>>();
    vf_stripped.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let elapsed_stripped = start_stripped.elapsed().as_nanos();

    println!("vf:          {:?}", vf);
    println!(
        "vf_stripped: {:?} in {:?} ns",
        vf_stripped, elapsed_stripped
    );
    println!("vf_cloned:   {:?} in {:?} ns", vf_cloned, elapsed_with_nan);

    // Random shuffled
    let mut v = vec![1, 2, 3, 4, 5, 6];
    v.shuffle(&mut thread_rng());
    println!("v shuffled: {:?}", v);

    // HashMap
    println!("{:-^20}", " COUNTING ".green().bold());
    let ballots = vec![
        "bob".to_string(),
        "john".to_string(),
        "anna".to_string(),
        "bob".to_string(),
        "anna".to_string(),
        "anna".to_string(),
    ];

    // For loop (need to clone to be able to create a String for entry
    // We coul dhave copied the ballots vector and use it in the for loop
    // Iterator
    let mut vote_counts_iter: HashMap<String, usize> = HashMap::new();
    let start_iter = Instant::now();
    for name in &ballots {
        vote_counts_iter
            .entry(name.clone())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let elapsed_iter = start_iter.elapsed().as_nanos();

    let mut vote_counts_for: HashMap<String, usize> = HashMap::new();
    let start_for = Instant::now();
    for name in &ballots {
        let count = vote_counts_for.entry(name.clone()).or_insert(0);
        *count += 1;
    }
    let elapsed_for = start_for.elapsed().as_nanos();

    println!(
        "{}",
        format!(
            "{}         vote_counts: {:?}, in {:?} ns",
            "For:".bold(),
            vote_counts_for,
            elapsed_for
        )
    );
    println!(
        "{}",
        format!(
            "{}        vote_counts: {:?}, in {:?} ns",
            "Iter:".bold(),
            vote_counts_iter,
            elapsed_iter
        )
    );

    // Simpler hash
    let mut fnv_vote_counts_iter: FnvHashMap<String, usize> = FnvHashMap::default();
    let fnv_start_iter = Instant::now();
    for name in &ballots {
        fnv_vote_counts_iter
            .entry(name.clone())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let fnv_elapsed_iter = fnv_start_iter.elapsed().as_nanos();
    println!(
        "{}",
        format!(
            "{}    vote_counts: {:?}, in {:?} ns",
            "FNV iter:".bold(),
            fnv_vote_counts_iter,
            fnv_elapsed_iter
        )
    );
    // Using AHash (not encryption secure)
    let mut ahash_vote_counts_iter: AHashMap<String, usize> = AHashMap::new();
    let ahash_start_iter = Instant::now();
    for name in &ballots {
        ahash_vote_counts_iter
            .entry(name.clone())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let ahash_elapsed_iter = ahash_start_iter.elapsed().as_nanos();
    println!(
        "{}",
        format!(
            "{}  vote_counts: {:?}, in {:?} ns",
            "Ahash iter:".bold(),
            ahash_vote_counts_iter,
            ahash_elapsed_iter
        )
    );

    println!("{:-^20}", " CHARS ".green().bold());
    {
        let radix = 16;
        {
            let digit = 21;
            let c = std::char::from_digit(digit, radix);
            println!("from_digit({}, {}) c: {:?}", digit, radix, c);
        }
        {
            let digit = 0;
            let c = std::char::from_digit(digit, radix);
            println!("from_digit({}, {}) c: {:?}", digit, radix, c);
        }
        {
            println!("{:?}", std::char::from_u32(0x9942));
        }
        {
            let digit = 15;
            let c = std::char::from_digit(digit, radix);
            println!("from_digit({}, {}) c: {:?}", digit, radix, c);
        }
    }

    println!("{:-^20}", " STRINGS ".green().bold());
    {
        let s = "Hello Rust";
        println!("slice: {}", &s[1..4]);
        let s_clone = s[1..4].to_owned();
        println!("s_clone: {}", s_clone);
    }
    {
        let s = "Hello Rust".to_string();
        println!("slice string: {}", &s[1..4]);
        let s_clone = s[1..4].to_owned();
        println!("s_clone: {}", s_clone);
    }
    {
        println!("{:?}", f64::from_str("1.2"));
        println!("{:?}", f64::from_str("true"));
        println!("{:?}", bool::from_str("true"));
        println!("{:?}", bool::from_str("TRUE"));
        println!("{:?}", char::from_str("abc"));
        println!("{:?}", char::from_str("a"));
    }
    {
        println!("{:?}", IpAddr::from_str("fe80::0000:3213:f3ff:f234:8789"));
        println!("{:?}", "fe80::0000:3213:f3ff:f234:8789".parse::<IpAddr>());
    }

    // Utf-8 conversion
    println!("{:=^20}", " UTF-8 ");
    {
        let good_utf8: Vec<u8> = vec![0xe9, 0x8c, 0x86];
        let good_string = String::from_utf8(good_utf8);
        // the vector has been been consumed;
        println!("good_utf8: {:?}", good_string);

        let bad_utf8: Vec<u8> = vec![0x9f, 0xf0, 0xa6, 0x80];
        let bad_string = String::from_utf8(bad_utf8);
        // the vector has been been consumed;
        println!("(Cow) bad_utf8: {:?}", bad_string);
    }
    {
        let good_utf8: Vec<u8> = vec![0xe9, 0x8c, 0x86];
        let string = String::from_utf8_lossy(&good_utf8);
        // the vector has been been consumed;
        println!("lossy: good_utf8: {:?}", string);

        let bad_utf8: Vec<u8> = vec![0x9f, 0xf0, 0xa6, 0x80];
        let bad_string = String::from_utf8_lossy(&bad_utf8);
        // the vector has been been consumed;
        println!("lossy: (Cow) bad_utf8: {:?}", bad_string);
    }
    // Utf-8 conversion
    println!("{:=^20}", " COW ");
    {
        let mut name = get_name();
        println!("name: {:?}", name);
        name.to_mut().push_str("!");
        println!("name: {:?}", name);
    }
    {
        let mut name = get_name();
        println!("name: {:?}", name);
        let s = write!(name.to_mut(), "!");
        println!("name: {:?}, null_or_error {:?}", name, s);
        name += "!";
        println!("name: {:?}", name);
    }
    {
        let mut name = get_name_using_into();
        println!("name: {:?}", name);
        name += "!";
        println!("name: {:?}", name);
    }
    println!("{:=^20}", " FORMATS ");
    {
        println!("{:.<40}", "Trailing");
        println!("{:.>40}", "Leading");
        println!("{:.^40}", "Centered");
        println!("{:.2e}", 100_000_000);
        println!("{:+012.2e}", 100_000_000);
        println!("{:>width$.limits$e}", 100_000_000, width = 12, limits = 2);
    }
}
