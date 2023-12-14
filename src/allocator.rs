use crate::utils::lazy::Lazy;
use crate::utils::mutex::Mutex;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

#[global_allocator]
static ALLOCATOR: Mutex<Lazy<Allocator>> = Mutex::new(Lazy::new(Allocator::new));

static mut HEAP: [u8; 65536] = [0; 65536];

struct Allocator {
    end: usize,
    next: usize,
}

impl Allocator {
    fn new() -> Self {
        let start = unsafe { HEAP.as_mut_ptr() as usize };
        let size = unsafe { HEAP.len() };

        Self {
            end: start + size,
            next: start,
        }
    }
}

unsafe impl GlobalAlloc for Mutex<Lazy<Allocator>> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();

        let start = match allocator.next % layout.align() {
            0 => allocator.next,
            remainder => allocator.next - remainder + layout.align(),
        };

        let end = match start.checked_add(layout.size()) {
            Some(end) => end,
            None => return null_mut(),
        };

        if end > allocator.end {
            null_mut()
        } else {
            allocator.next = end;
            start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
