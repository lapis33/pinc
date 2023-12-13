use crate::mutex::Mutex;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

#[global_allocator]
static ALLOCATOR: Mutex<Allocator> = Mutex::new(Allocator::empty());

static mut HEAP: [u8; 65536] = [0; 65536];

struct Allocator {
    end: usize,
    next: usize,
}

impl Allocator {
    const fn empty() -> Self {
        Self { end: 0, next: 0 }
    }
}

unsafe impl GlobalAlloc for Mutex<Allocator> {
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

pub fn init() {
    let start = unsafe { HEAP.as_mut_ptr() as usize };
    let size = unsafe { HEAP.len() };

    let mut allocator = ALLOCATOR.lock();
    allocator.end = start + size;
    allocator.next = start;
}
