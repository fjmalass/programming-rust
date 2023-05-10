fn smallest<'a>(list: &'a [i32]) -> &'a i32 {
    let mut s = &list[0];
    for r in &list[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let r = smallest(&v);
    println!("smallest r: {:#?}", r);
    v[2] = -10;
    let r = smallest(&v);
    println!("smallest r: {:#?}", r);

    let mut w = vec![1, 2, 3, 4, 5];
    {
        let r = &mut w[1..];
        _ = std::mem::replace(&mut r[0], 10);
        println!("-- mem::replace r[0], i.e. w[1] --");
        println!("r: {:?}", r);
        println!("-- mut ref assign r[1], i.e., w[2] --");
        r[1] = 20;
        println!("r: {:?}", r);
    }

    let aside = w;
    println!("-- check w outside--");
    println!("aside: {:?}", aside);
}
