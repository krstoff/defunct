use super::{PAGE_SIZE, ARENA_SIZE};

const ARENA_BITS_SIZE: usize = ARENA_COUNT / 8;
const ARENA_COUNT: usize = ARENA_SIZE / PAGE_SIZE;
type ArenaBits = [u8; ARENA_BITS_SIZE];

// 64MiB arena of 8KiB pages with a bitmap tracking page allocation.
pub struct Arena {
    base: *mut u8,
    count: usize,
    bits: ArenaBits,
}

impl Arena {
    pub fn new() -> Arena {
        use std::alloc::*;
        unsafe {
            let layout = Layout::from_size_align(
                ARENA_SIZE, PAGE_SIZE
            ).expect("Arena allocation was misaligned.");
            let base = alloc(layout);
            Arena {
                base, bits: [0u8; ARENA_BITS_SIZE], count: 0
            }
        }
    }

    pub fn base(&mut self) -> *mut u8 {
        self.base
    }

    pub fn bits(&self) -> &ArenaBits {
        &self.bits
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn try_alloc(&mut self, npages: usize) -> Option<*mut u8> {
        if npages == 0 || npages > ARENA_COUNT {
            panic!("Tried to allocate a span with a bad number of pages.");
        }

        if npages + self.count > ARENA_COUNT {
            return None;
        }

        let total_pages = ARENA_COUNT;
        let bits = &mut self.bits;

        let mut start = 0;
        'outer: while start < total_pages {
            let mut count = 0;
            for i in start..total_pages {
                if (bits[i / 8] & (1 << (i % 8))) != 0 {
                    start = i + 1;
                    continue 'outer;
                }
                count += 1;

                if count >= npages {
                    // mark bits
                    for i in start..(start + count) {
                        bits[i / 8] |= 1 << (i % 8);
                    }
                    self.count += npages;
                    let addr = self.base as usize + start * PAGE_SIZE;
                    return Some(addr as *mut u8)
                }
            }
            break;
        }

        None
    }

    pub fn dealloc(&mut self, start: *mut u8, npages: usize) {
        let offset = start as isize - self.base as isize;
        assert!(offset >= 0 && offset < ARENA_SIZE as isize);
        assert!(npages * PAGE_SIZE + (offset as usize) <= ARENA_SIZE);

        let page = offset as usize / PAGE_SIZE;
        for i in page..page + npages {
            self.bits[i / 8] &= !(1 << i % 8);
        }
        self.count -= npages;
    }

    pub fn print_alloc_bits(&self, max: usize) {
        let n = if max == 0 { ARENA_BITS_SIZE } else { max.div_ceil(8) };
        for i in 0..n {
            print!("{:08b}", self.bits[i].reverse_bits());
        }
        println!("");
    }
}


impl Drop for Arena {
    fn drop(&mut self) {
        use std::alloc::*;
        let layout = Layout::from_size_align(
                ARENA_SIZE, PAGE_SIZE
        ).expect("Arena allocation was misaligned.");
        unsafe {
            std::alloc::dealloc(self.base, layout);
        }
    }
}