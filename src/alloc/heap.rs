use std::ptr::NonNull;
use allocator_api2::alloc as alloc;

use super::{NUM_SIZE_CLASSES, MAX_SMALL_OBJ_SIZE};
use super::span::{Span, get_size_class, get_obj_size, get_alloc_pages};
use super::Arena;

// TODO: Implement partial and full as Chunked Lists
pub struct SpanSet {
    pages: usize,
    obj_size: usize,
    partial: Vec<Span>,
    full: Vec<Span>
}

impl SpanSet {
    fn new(class: usize, obj_size: usize) -> SpanSet {
        let pages = get_alloc_pages(class);
        SpanSet { pages, obj_size, partial: vec![], full: vec![] }
    }

    fn is_full(&self) -> bool {
        self.partial.len() == 0
    }

    fn add_span(&mut self, span: Span) {
        assert!(self.pages == span.pages as usize);
        assert!(self.obj_size == span.obj_size as usize);
        self.partial.push(span);
    }

    // Returns None only if span_set is full and needs a new span.
    fn alloc(&mut self) -> Option<*mut u8> {
        if self.partial.len() == 0 { return None }
        let span = self.partial.last_mut().unwrap();
        let val = span.alloc();
        if span.is_full() {
            let span = self.partial.pop().unwrap();
            self.full.push(span);
        }
        val
    }
}

pub struct HeapInner {
    // TODO: Doubly linked list for this part?
    page_arenas: Vec<Arena>,
    span_sets: Vec<SpanSet>
}

impl HeapInner {
    pub fn new() -> HeapInner {
        let mut span_sets = vec![];
        for i in 0..NUM_SIZE_CLASSES {
            span_sets.push(
                SpanSet::new(i, get_obj_size(i))
            );
        }
        let page_arenas = vec![Arena::new()]; 
        HeapInner { page_arenas, span_sets }
    }
    
    pub fn alloc(&mut self, size: usize) -> *mut u8 {
        if size > super::MAX_SMALL_OBJ_SIZE {
            return self.alloc_large(size);
        }

        let size_class = super::span::get_size_class(size);
        // TODO: cacheing last span
        if self.span_sets[size_class].is_full() {
            let new_span = self.alloc_span(size_class);
            self.span_sets[size_class].add_span(new_span);
        }
        let ptr = self.span_sets[size_class].alloc().unwrap();
        return ptr;

    }

    // TODO: just default to system calculator?
    fn alloc_large(&mut self, size: usize) -> *mut u8 {
        unimplemented!()
    }

    // Gets a new span reservation from a page_arena, allocating a new arena if necessary.
    fn alloc_span(&mut self, class: usize) -> Span {
        for arena in self.page_arenas.iter_mut() {
            if let Some(base) = arena.try_alloc(get_alloc_pages(class)) {
                return Span::new(base, class)
            }
        }
        // if we got here, there must not be enough arenas
        let mut arena = Arena::new();
        let base = arena.try_alloc(get_alloc_pages(class))
            .expect("Allocation from a fresh arena should not have failed.");
        self.page_arenas.push(arena);
        Span::new(base, class)
    }
}

pub struct Heap;

impl Heap {
    pub fn alloc(size: usize) -> *mut u8 {
        super::HEAP.with(|heap|
            heap.borrow_mut().alloc(size)
        )
    }
}

unsafe impl alloc::Allocator for Heap {
    fn allocate(&self, layout: std::alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
        let ptr = super::HEAP.with(|heap|
            heap.borrow_mut().alloc(layout.size())
        );
        let allocation = unsafe { std::slice::from_raw_parts_mut(ptr, layout.size()) };
        Ok(NonNull::from(allocation))
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: std::alloc::Layout) {
        // No op for now :)
    }
}