use colored::*;
use std::rc::Rc;
use std::ops::{Deref, DerefMut};

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
        let mut s = Selector{current:0,values : vec!["one".to_string(),"two".to_string()]};
        println!("s = {:?}, deref: {:?}", s, *s);
        s.current = 1;
        println!("s = {:?}, {:p}, values: {:p},  deref: {:?}", s, &s, &s.values, *s);
        println!("s = {}", &(*s)[1..]);
        *s = "three".to_string();
        println!("s = {:?}, {:p}, values: {:p},  deref: {:?}", s, &s, &s.values, *s);
        s.current = 10;
        // panick as only 2 values
        // println!("s = {:?}, {:p}, values: {:p},  deref: {:?}", s, &s, &s.values, *s);
    }

    println!("{}", "--- 05. Default ---".green());
    println!("{}", "--- 06. AsRef ---".green());
    println!("{}", "--- 07. Borrow ---".green());
    println!("{}", "--- 08. From/Into ---".green());
    println!("{}", "--- 09. TryFrom/TryInto ---".green());
    println!("{}", "--- 10. ToOwned ---".green());
}
