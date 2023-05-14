use std::borrow::Borrow;
use std::io::Write;
use std::rc::Rc;

// Generics better than trait objects
fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"Hello, world! With Generics")?;
    out.flush()
}

fn main() {
    println!("Traits - Generics!");
    let mut buf: Vec<u8> = vec![];
    let mut writer: &mut dyn Write = &mut buf;
    let val = say_hello(&mut writer);
    println!("buf: {:?}: val: {:?}", buf.len(), val);

    // using fat pointers (with vtable)
    let mut w: Box<&mut dyn Write> = Box::new(&mut buf);
    let out  = say_hello(&mut w);
    println!("out: {:?}", out);
}
