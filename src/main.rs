use std::time::Duration;

mod values;
mod alloc;
mod ptr;

fn main() {
    let mut heap = alloc::Heap::new();
    let mut ptrs = vec![];
    for i in 0..10000 {
        let ptr = heap.alloc(16) as *mut u128;
        unsafe { *ptr = i as u128; }
        ptrs.push(ptr);
    }
    
    println!("{}", (ptrs[9999] as usize - ptrs[0] as usize) / 16);
}
