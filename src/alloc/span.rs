use super::PAGE_SIZE;
use super::NUM_SIZE_CLASSES;

// These are lifted straight from Go's source.
// Go targets 67 size classes between 8B and 32KB that minimizes waste to at most 12.5% of memory.
const CLASS_TO_SIZE: [u64; NUM_SIZE_CLASSES] = [
    0, 16, 32, 48, 64, 80, 96, 112, 128, 144,
    160, 176, 192, 208, 224, 240, 256, 288, 320,
    352, 384, 416, 448, 480, 512, 576, 640, 704,
    768, 896, 1024, 1152, 1280, 1408, 1536, 1792, 
    2048, 2304, 2688, 3072, 3200, 3456, 4096, 4864,
    5376, 6144, 6528, 6784, 6912, 8192, 9472, 9728, 
    10240, 10880, 12288, 13568, 14336, 16384, 18432, 
    19072, 20480, 21760, 24576, 27264, 28672, 32768
];

const NUM_SMALL_BUCKETS: usize = (1024 + 7) / 8;
const NUM_MEDIUM_BUCKETS: usize = (32768 - 1024 + 127) / 128;
const TOTAL_BUCKETS: usize = NUM_SMALL_BUCKETS + NUM_MEDIUM_BUCKETS;

const fn generate_size_to_class() -> [u64; TOTAL_BUCKETS] {
    let mut table = [0u64; TOTAL_BUCKETS];
    let mut class_idx = 0;

    // Small sizes, 8-byte rounding
    let mut bucket = 0;
    while bucket < NUM_SMALL_BUCKETS {
        let size = bucket as u64 * 8;
        if size > CLASS_TO_SIZE[class_idx] {
            class_idx += 1;
        }
        table[bucket] = class_idx as u64;
        bucket += 1;
    }

    // Medium sizes, 128-byte rounding
    let mut offset = 0;
    while offset < NUM_MEDIUM_BUCKETS {
        let size = offset as u64 * 128 + 1024;
        if size > CLASS_TO_SIZE[class_idx] {
            class_idx += 1;
        }
        let bucket = offset + NUM_SMALL_BUCKETS;
        table[bucket] = class_idx as u64;
        offset += 1;
    }
    
    table
}

/// Static table mapping increments of 8 or 128 bytes to static size classes.
const SIZE_TO_CLASS: [u64; TOTAL_BUCKETS] = generate_size_to_class();

// Uses constant lookup tables to calculate the alloc-class of an allocation size.
pub fn get_size_class(size: usize) -> usize {
    if size > 32768 {
        return 0;
    }
    let bucket = 
        if size <= 1024 {(size + 7) / 8 }
        else { NUM_SMALL_BUCKETS + (size - 1024 + 127) / 128 };
    return SIZE_TO_CLASS[bucket] as usize;
}

pub fn get_obj_size(class: usize) -> usize {
    return CLASS_TO_SIZE[class] as usize;
}

pub fn get_alloc_pages(class: usize) -> usize {
    return CLASS_TO_ALLOC_PAGES[class] as usize;
}

// For any given size class, we must pick the number of pages to allocate to a span.
// Go tries to keep the given waste for a size class to be around 1/8 at max.
const fn generate_class_to_alloc_pages() -> [usize; NUM_SIZE_CLASSES] {
    let mut table = [0; NUM_SIZE_CLASSES];
    table[0] = 0; // Reserved for large allocations
    let mut class_idx = 1;
    while class_idx < NUM_SIZE_CLASSES {
        if (CLASS_TO_SIZE[class_idx] as usize) <= PAGE_SIZE {
            let obj_size = CLASS_TO_SIZE[class_idx] as usize;
            let mut num_pages: usize = 1;
            loop {
                let waste = num_pages * PAGE_SIZE % obj_size;
                if 8 * waste < num_pages * PAGE_SIZE {
                    break;
                }
                num_pages += 1;
            }
            table[class_idx] = num_pages;
        } else {
            let obj_size = CLASS_TO_SIZE[class_idx] as usize;
            let mut num_objects = 1;
            loop {
                let num_pages = (obj_size * num_objects).div_ceil(PAGE_SIZE);
                let waste = num_pages * PAGE_SIZE % obj_size;
                if 8 * waste < num_pages * PAGE_SIZE {
                    break;
                }
                num_objects += 1;
            }
            table[class_idx] = (obj_size * num_objects).div_ceil(PAGE_SIZE);
        }

        class_idx += 1;
    }

    table
}

const CLASS_TO_ALLOC_PAGES: [usize; NUM_SIZE_CLASSES] = generate_class_to_alloc_pages();

// For debugging size-class generation.
pub fn print_size_classes() {
    println!("class\tsize\tpages\twaste_%\tobjects");
    for i in 1..NUM_SIZE_CLASSES {
        let span_size = CLASS_TO_ALLOC_PAGES[i] * PAGE_SIZE;
        let waste = span_size % CLASS_TO_SIZE[i] as usize;
        let waste_p = waste as f64 / span_size as f64;
        println!("{}\t{}\t{}\t{:.2}\t{}", i, CLASS_TO_SIZE[i], CLASS_TO_ALLOC_PAGES[i], waste_p, span_size / CLASS_TO_SIZE[i] as usize);
    }
}

// The most amount of objects a span can hold is going to be 512. This bitfield fits perfectly in cache.
const SPAN_BITS_SIZE: usize = 64;
type SpanBits = [u8; SPAN_BITS_SIZE];

// Metadata for a continuous run of pages for allocating objects of size obj_size
// TODO: Allocate Spanbits separately instead of inline? 
pub struct Span {
    pub base: *mut u8,
    pub pages: u8,
    pub obj_size: u16,
    pub capacity: u16,
    pub count: u16,
    pub alloc_bits: SpanBits,
}

impl Span {
    pub fn new(base: *mut u8, class: usize) -> Span {
        let pages = get_alloc_pages(class);
        let obj_size = get_obj_size(class);
        let capacity = pages * PAGE_SIZE / obj_size;
        Span {
            base, 
            pages: pages as u8,
            obj_size: obj_size as u16,
            capacity: capacity as u16,
            count: 0,
            alloc_bits: [0u8; SPAN_BITS_SIZE ] 
        }
    }

    pub fn alloc(&mut self) -> Option<*mut u8> {
        if self.count == self.capacity {
            return None;
        }

        for i in 0..self.capacity as usize {
            if self.alloc_bits[i / 8] & 1 << (i % 8) == 0 {
                self.count += 1;
                self.alloc_bits[i / 8] |= 1 << i % 8;
                let offset = self.obj_size as usize * i;
                return Some(unsafe { self.base.add(offset) })
            }
        }

        panic!("During slot allocation, discovered span count was corrupted");
    }

    pub fn is_full(&self) -> bool {
        self.count == self.capacity
    }
}