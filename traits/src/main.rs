use std::io::Write;
use std::rc::Rc;
use std::borrow::Borrow;


// Generics better than trait objects
fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"Hello, world! With Generics")?;
    out.flush()
}

fn main() {
    println!("Traits - Generics!");
    let mut buf: Vec<u8> = vec![];
    let mut writer: &mut dyn Write = &mut buf;
    _ = say_hello(&mut writer).unwrap();
    println!("buf: {:?}", buf.len());


    // using fat pointers (with vtable)
    let mut w: Box<&mut dyn Write> = Box::new(&mut buf);
    _ = say_hello(&mut w).unwrap();
    println!("buf with Fat pointer: {:?}", buf.len());




}
