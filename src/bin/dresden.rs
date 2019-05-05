use parken::{Dresden, Parken, Lot};

fn main()  {
    for Lot{name, capacity, free} in &Dresden::get_lots().unwrap() {
        if let Some(capacity) = capacity {
            println!("{:25} {:5}/{:5}", name, free.unwrap_or(0), capacity);
        }
    }
}
