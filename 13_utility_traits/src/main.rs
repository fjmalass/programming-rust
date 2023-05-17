use colored::*;
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::rc::Rc;
use std::io::{ ErrorKind, Error};

#[derive(Debug)]
struct Appelation {
    name: String,
    nicknames: Vec<String>,
}

impl Drop for Appelation {
    fn drop(&mut self) {
        println!("   {}",  "DROP: Should only be implemented when resources are 'exotic', like os or other c libraries".yellow());
        println!("   DROP: Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            println!("   DROP:  (AKA {})", self.nicknames.join(", "));
        }
        println!("");
    }
}

// Selection keeps the index that is currently active
#[derive(Debug)]
pub struct Selector<T> {
    pub current: usize,
    pub values: Vec<T>,
}

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.values[self.current]
    }
}
impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values[self.current]
    }
}

/// DEFAULT
#[derive(Debug)]
pub struct Params {
    pub name: String,
    pub is_active: bool,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            is_active: true,
            width: 640,
            height: 480,
            depth: 1,
        }
    }
}

// by default
#[derive(Debug, Default)]
pub struct ValuesZeroByDefault {
    pub a: u32,
    pub b: u32,
    pub c: u32,
}

// Using cow (borrowed  vs. owned)
fn describe(error: &Error) -> Cow<'static, str> {
    match error.kind() {
        ErrorKind::NotFound  => format!("file not found: {:?}", error.get_ref()).into(), // Owned
        _ => "unknown error".into(),
    }
}

fn main() {
    println!("{}", "--- UTILITY TRAITS ---".green());
    println!("{}", "--- 01. Drop ---".green());
    {
        let mut a = Appelation {
            name: "Apple".to_string(),
            nicknames: vec!["Pomme".to_string(), "Manzana".to_string()],
        };
        println!("Created {}", a.name);
        a.nicknames.push("Apfel".to_string());
        println!("Pushed Apfel");
        println!("About to exit scope");
    }
    println!("- Another drop -");
    {
        let p;
        {
            let q = Appelation {
                name: "Orange".to_string(),
                nicknames: vec!["Orangina".to_string(), "Yellow".to_string()],
            };
            println!("Created q {:?}", q);
            p = q;
            println!("Moved to p {:?}", p);
        }
        println!("About to exit scope");
    }

    println!("{}", "--- 02. Sized ---".green());
    println!("{}", "- Sized is a trait for values that have same size in memory, e.g, f32, Vec<T> (as Fat Pointer) -".yellow());
    println!(
        "{}",
        "- UnSized, are `str` with string litterals, array slice `[T]`, or `dyn` traits -".yellow()
    );
    println!(
        "{}",
        "- To deal with them in variabls, use references e.g., `&str` or `Box<dyn trait>` -"
            .yellow()
    );

    println!("{}", "--- 03. Clone ---".green());
    println!(
        "{}",
        "- Clone is a trait for values that can be copied, e.g., `Vec<T>` -".yellow()
    );

    println!(" clone with RC -> Only increase counter by 1 --");
    {
        use std::rc::Rc;
        let s: Rc<String> = Rc::new("shirataki".to_string());
        {
            let t = s.clone();
            {
                let u = s.clone();
                println!(
                    "s = {:?}, {:p}, t = {:?}, {:p}, u = {:?}, {:p}",
                    Rc::strong_count(&s),
                    s,
                    Rc::strong_count(&t),
                    t,
                    Rc::strong_count(&u),
                    u
                );
            }
            println!(
                "s = {:?}, {:p}, t = {:?}, {:p}",
                Rc::strong_count(&s),
                s,
                Rc::strong_count(&t),
                t
            );
        }
        println!("s = {:?}, {:p}", Rc::strong_count(&s), s);
    }
    println!("{}", "--- 03.a Copy ---".green());
    println!("- Copy requres bit-by-bit copy (no heap) -");

    println!("{}", "--- 04. Deref ---".green());
    println!(
        "{}",
        " - how to get the * and . operator like with Box".yellow()
    );
    // box int32
    {
        let mut b = Box::<i32>::new(5);
        println!("b = {}, ptr: {:p}", b, b);
        println!("*b = {:?}", *b);
        *b = 10;
        println!("b = {}, {:p}", b, b);
    }
    // box struct
    {
        let mut a = Box::<Appelation>::new(Appelation {
            name: "Apple".to_string(),
            nicknames: vec!["Pomme".to_string(), "Manzana".to_string()],
        });
        println!("a = {:?}, ptr: {:p}", a, a);
        a.name = "Orange".to_string(); // shortcut of (*a).name
        println!("*a = {:?}", *a);
    }
    {
        let mut s = Selector {
            current: 0,
            values: vec!["one".to_string(), "two".to_string()],
        };
        println!("s = {:?}, deref: {:?}", s, *s);
        s.current = 1;
        println!(
            "s = {:?}, {:p}, values: {:p},  deref: {:?}",
            s, &s, &s.values, *s
        );
        println!("s = {}", &(*s)[1..]);
        *s = "three".to_string();
        println!(
            "s = {:?}, {:p}, values: {:p},  deref: {:?}",
            s, &s, &s.values, *s
        );
        s.current = 10;
        // panick as only 2 values
        // println!("s = {:?}, {:p}, values: {:p},  deref: {:?}", s, &s, &s.values, *s);
    }

    println!("{}", "--- 05. Default ---".green());
    {
        let p = Params::default();
        println!("p = {:?}", p);
    }
    {
        println!(
            "{}",
            "- Default with only 1 changed using ..default)".yellow()
        );
        let p = Params {
            name: "Width: 1080".to_string(),
            width: 1080,
            ..Params::default()
        };
        println!("p = {:?}", p);
    }
    {
        println!("{}", "Default with box".yellow());
        let p = Box::<Params>::default();
        println!("p = {:?}, ptr: {:p} ", p, p);
    }

    {
        let z = ValuesZeroByDefault::default();
        println!("z = {:?}", z);
    }

    println!("{}", "--- 06. AsRef / AsRefMut  ---".green());
    println!(
        "{}",
        "- Used for functions for more flexible argument types".yellow()
    );
    println!(
        "{}",
        "- e.g., `fn read_file<T: AsRef<Path>>(path: T) -> Result<String>` -".yellow()
    );

    println!("{}", "--- 07. Borrow / BorrowMut ---".green());
    println!(
        "{}",
        "- More restrictive for Hash Tables and Trees than AsRef (particularly with String)"
            .yellow()
    );
    let mut hashtable = HashMap::<String, usize>::new();
    hashtable.insert("one".to_string(), 1);
    hashtable.insert("two".to_string(), 2);
    println!("Hashtable: {:?}", hashtable);
    let key = "one".to_string();
    println!("key: {:?}, Hash[key]: {}", key, hashtable[&key]);
    println!("key as string litterl:  Hash[key]: {}", hashtable["two"]);

    println!("{}", "--- 08. From/Into ---".green());
    println!(
        "{}",
        "- From/Into are more expensive than  AsRef/AsMut they often do allocation/ data checking-"
            .yellow()
    );
    println!(
        "{}",
        "- From/Into should not be implemented if err, like f64 into f32, Use try_into -".yellow()
    );

    println!("{}", "--- 09. TryFrom/TryInto ---".green());
    println!("{}", "- Returns Resut, like f64 -> f32 -".yellow());
    {
        let large: usize = 2_000_000_000_000;
        let missed: i16 = large.try_into().unwrap_or_else(|_| {
            if large >= 0 {
                std::i16::MAX
            } else {
                std::i16::MIN
            }
        });
        println!("large: {}, missed: {}", large, missed);
    }

    println!("{}", "--- 10. ToOwned ---".green());
    println!(
        "{}",
        "- ToOwned is like Clone but can return a Path or String from a reference -".yellow()
    );
    let s = "hello".to_string();
    let p = std::path::Path::new("more").to_owned();
    println!(
        "{}",
        "--- 11. Borrow and ToOwned using Copy on Write (CoW) ---".green()
    );
    println!(
        "{}",
        "- CoW is a smart pointer that only copies when needed -".yellow()
    );


}

