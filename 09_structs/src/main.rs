use core::cell::Cell;
use std::cmp::PartialOrd;
use std::rc::Rc;
use std::time::Instant;



#[derive(Copy, Clone, Debug)]
pub enum BroomIntent {
    FetchWater,
    DumpWater,
}

#[derive(Debug)]
pub struct Broom {
    pub name: String,
    pub height: u32,
    pub health: u32,
    pub position: (f32, f32, f32),
    pub intent: BroomIntent,
}

impl Broom {
    const CENTER: (f32, f32, f32) = (0.0, 0.0, 0.0);
    const NAME: &'static str = "BROOM";
}

fn chob(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom {
        height: b.height / 2,
        ..b
    }; // grab Broom and replace with two
    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1
    }; // replicate the other elements
    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}

pub struct Queue<T> {
    older: Vec<T>,   // older elements, eldest last.
    younger: Vec<T>, // yourer elements, youngest last.
}

impl<T> Queue<T> {
    /// Create empty queue
    pub fn new() -> Queue<T> {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }
    ///
    /// Push charter to the back of the queue.
    pub fn push(&mut self, c: T) {
        self.younger.push(c);
    }

    /// is_empty
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    /// Popo a chacater off the front of the queue.
    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            // Bring the elements in younger over to older, and put them in the promised order.
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        // Now older is guaranteed to have something. Vec's pop method
        // already returns an Option, so we're set.
        self.older.pop()
    }
    // Split into younter and older but consume self in the process.
    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
/// References to greatest and least elements
#[derive(Debug)]
pub struct ExtremaRef<'elt, T> {
    pub greatest: &'elt T,
    pub least: &'elt T,
}

/// Find the greatest and least numbers in a slice.
pub fn find_extrema_ref<'s, T>(slice: &'s [T]) -> ExtremaRef<'s, T>
where
    T: PartialOrd,
{
    let mut greatest = &slice[0];
    let mut least = &slice[0];
    for i in 1..slice.len() {
        if slice[i] < *least {
            least = &slice[i];
        }
        if slice[i] > *greatest {
            greatest = &slice[i];
        }
    }
    ExtremaRef::<T> { greatest, least }
}

#[derive(Debug)]
pub struct Extrema<T> {
    pub greatest: T,
    pub least: T,
}

/// Find the greatest and least numbers in a slice.
pub fn find_extrema<T>(slice: &[T]) -> Extrema<T>
where
    T: PartialOrd + Copy,
{
    let mut greatest = slice[0];
    let mut least = slice[0];
    for i in 1..slice.len() {
        if slice[i] < least {
            least = slice[i];
        }
        if slice[i] > greatest {
            greatest = slice[i];
        }
    }
    Extrema::<T> { greatest, least }
}

/// Polynomial to have constant parameters

#[derive(Copy, Clone, Debug)]
pub struct Polynomial<const N: usize> {
    coeffs: [f64; N],
}

impl<const N: usize> Polynomial<N> {
    pub fn new(coeffs: [f64; N]) -> Self {
        Self { coeffs }
    }

    pub fn eval(&self, x: f64) -> f64 {
        let mut sum = 0.0;
        for i in 0..N {
            sum += self.coeffs[i] * x.powi(i as i32);
        }
        sum
    }
}

#[derive(Clone, Debug)]
// Everything will be immutable
pub struct Robot {
    pub name: String,
    pub power_level: u32,
    pub hardware_error_count: Cell<u32>, // allows to mutate the value:w
}

#[derive(Clone, Debug)]
pub struct RobotSensor {
    pub robot: Rc<Robot>,
    pub range: f64,
    pub angle: f64,
}

fn main() {
    let b = Broom {
        name: Broom::NAME.to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater,
    };
    println!("b is {:?}", b);
    println!("CENTER: {:?}", Broom::CENTER);

    let (b1, b2) = chob(b);
    println!("b has been consumed");
    println!("b1 is {:?}", b1);
    println!("b2 is {:?}", b2);
    println!("-----------");
    println!("-- Queue --");
    println!("-----------");

    let mut q = Queue::<char> {
        older: Vec::new(),
        younger: Vec::new(),
    };
    q.push('0');
    q.push('1');
    println!("q.is_empty() is {:?}", q.is_empty());
    println!("q.pop() is {:?}", q.pop());
    q.push('=');
    println!("q.pop() is {:?}", q.pop());
    println!("q.pop() is {:?}", q.pop());
    println!("q.pop() is {:?}", q.pop());
    println!("q.is_empty() is {:?}", q.is_empty());

    println!("-----------");
    println!("-- split --");
    println!("-----------");
    let mut q = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };
    q.push('0');
    q.push('1');
    q.push('2');
    _ = q.pop();
    q.push('=');
    let (younger, older) = q.split();
    println!("younger is {:?}", younger);
    println!("older is {:?}", older);

    println!("----------");
    println!("-- box --");
    println!("----------");
    let mut bq = Box::new(Queue::<u8>::new());
    // borrows a & Queue for the duratio of the call
    println!("bq.is_empty() is {:?}", bq.is_empty());
    // borrows a & Queue for the duratio of the call
    bq.push(1);
    println!("bq.is_empty() is {:?}", bq.is_empty());

    println!("----------------");
    println!("-- Polynomial --");
    println!("----------------");
    let a = [0., 1., 10.];
    let p = Polynomial::new(a);
    // extrema.least = &100;
    println!("polynomial is {:?}", p);
    let x = 2.;
    println!("p({}) = {}", x, p.eval(x));

    println!("---------------------------------");
    println!("-- Testing Cell With Immutable --");
    println!("---------------------------------");
    let r = Robot {
        name: "Bender".to_string(),
        power_level: 100,
        hardware_error_count: Cell::new(0),
    };
    println!("r is {:?}", r);
    println!(" adding a hardware error even when r is immutable");
    r.hardware_error_count.set(1);
    println!("r is {:?}", r);

    println!("-----------------");
    println!("-- Extrema Ref --");
    println!("-----------------");
    let a: Vec<i64> = (0..600_000_000).map(|x| x as i64).collect();
    let start = Instant::now();
    let extrema_ref = find_extrema_ref(&a);
    let elapsed = start.elapsed();
    println!("extrema: {:?}", type_of(&a));
    println!("extrema_ref calculation: {:?}", elapsed);
    println!("extrema_ref is {:p}", extrema_ref.least);
    println!("extrema_ref is {:?}, {:p}", extrema_ref, &extrema_ref);

    println!("----------------");
    println!("-- With float --");
    println!("----------------");
    let a_f: Vec<f64> = (0..600_000_000).map(|x| x as f64).collect();
    let start = Instant::now();
    let extrema_ref = find_extrema_ref(&a_f);
    let elapsed = start.elapsed();
    println!("extrema: {:?}", type_of(&a_f));
    println!("extrema_ref calculation: {:?}", elapsed);
    println!("extrema_ref is {:p}", extrema_ref.least);
    println!("extrema_ref is {:?}, {:p}", extrema_ref, &extrema_ref);

    println!("-------------");
    println!("-- Extrema --");
    println!("-------------");
    let start = Instant::now();
    let extrema = find_extrema(&a);
    let elapsed = start.elapsed();
    println!("extrema calculation: {:?}", elapsed);
    println!("extrema is {:?}", extrema.least);
    println!("extrema is {:?}, {:p}", extrema, &extrema);

    println!("----------------");
    println!("-- With float --");
    println!("----------------");
    let start = Instant::now();
    let extrema = find_extrema(&a_f);
    let elapsed = start.elapsed();
    println!("extrema calculation: {:?}", elapsed);
    println!("extrema is {:?} {:p}", extrema.least, &extrema.least);
    println!("extrema is {:?}, {:p}", extrema, &extrema);
}
