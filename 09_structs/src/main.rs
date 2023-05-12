use std::cmp::{PartialOrd, Ord};

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

/// References to greatest and least elements
#[derive(Debug)]
pub struct ExtremaRef<'elt, T> {
    pub greatest: &'elt T,
    pub least: &'elt T,
}

/// Find the greatest and least numbers in a slice.
pub fn find_extremas<'s, T>(slice: &'s [T]) -> ExtremaRef<'s, T>
where T: PartialOrd,
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

fn main() {
    let b = Broom {
        name: "Zero".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater,
    };
    println!("b is {:?}", b);

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

    println!("--------------");
    println!("-- Extremas --");
    println!("--------------");
    let a = vec![0, 1, 10, -10, 100];
    let extrema = find_extremas(&a);
    // extrema.least = &100;
    println!("extrema is {:p}", extrema.least);
    println!("extrema is {:?}, {:p}", extrema, &extrema);

    println!("----------------");
    println!("-- With float --");
    println!("----------------");
    let a = vec![0., 1., 10., -10., 100.];
    let extrema = find_extremas(&a);
    // extrema.least = &100;
    println!("extrema is {:p}", extrema.least);
    println!("extrema is {:?}, {:p}", extrema, &extrema);
}
