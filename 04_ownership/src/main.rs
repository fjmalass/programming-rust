use std::rc::Rc;

#[derive(Debug)]
struct Person {
    name: Option<String>,
    dob: u32,
}

fn main() {
    let s = vec!["hello".to_string(), "world".to_string()];
    println!("s = {:?}", s);
    let t = &s;
    println!("t = {:?}", t);
    println!("s = {:?}", s);

    let mut composers = Vec::<Person>::new();
    composers.push(Person {
        name: Some("Palestrina".to_string()),
        dob: 1525,
    });
    composers.push(Person {
        name: Some("Myself".to_string()),
        dob: 1900,
    });
    println!("-----------------------");
    println!("== take (for Option) ==");
    println!("-----------------------");
    let first_name = composers[0].name.take(); // the composers[0].name is changed to None
    println!("first_name = {:#?}", first_name);
    let first_dob = composers[0].dob;
    println!("first_dob = {:#?}", first_dob); // ok as copy
    println!("composers = {:#?}", composers);

    println!("-------------------------------");
    println!("== mem::replace (for Option) ==");
    println!("-------------------------------");
    let second_name = std::mem::replace(&mut composers[1].name, Some("Bach".to_string()));
    println!("second_name = {:#?}", second_name);
    println!("composers = {:#?}", composers);

    println!("---------------------------");
    println!("== Reference Counting Rc ==");
    println!("---------------------------");
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t = s.clone(); //add`1 reference count
    let u = s.clone(); //add`1 reference count
    println!(
        "s = {:?}, t = {:?}, u = {:?}",
        Rc::strong_count(&s),
        Rc::strong_count(&t),
        Rc::strong_count(&u)
    );
    println!("-- Adding v within a scope --");
    {
        let v = u.clone(); //add`1 reference count
        println!(
            "s = {:?}, t = {:?}, u = {:?}, v = {:?}",
            Rc::strong_count(&s),
            Rc::strong_count(&t),
            Rc::strong_count(&u),
            Rc::strong_count(&v)
        );
        // as v is dropped the reference count is decreased by 1
    }
    println!("-- Out of scope --");
    println!(
        "s = {:?}, t = {:?}, u = {:?}",
        Rc::strong_count(&s),
        Rc::strong_count(&t),
        Rc::strong_count(&u)
    );
    println!("-- std::mem::drop --");
    std::mem::drop(s); // as s is dropped the reference count is decreased by 1
    println!(
        "t = {:?}, u = {:?}",
        Rc::strong_count(&t),
        Rc::strong_count(&u)
    );
}
