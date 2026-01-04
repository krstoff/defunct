mod heap;
mod arena;
mod span;

const ARENA_SIZE: usize = 1 << 26;
const PAGE_SIZE: usize = 1 << 13;
const NUM_SIZE_CLASSES: usize = 66;
const MAX_SMALL_OBJ_SIZE: usize = 32768;

use arena::Arena;
use span::Span;

pub use heap::Heap;
pub use span::print_size_classes;