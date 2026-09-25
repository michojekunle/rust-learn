pub mod bump;
pub mod freelist;
pub mod linked_list;
use spin::{Mutex, MutexGuard};

use bump::BumpAllocator;
use linked_list::LinkedListAllocator;

// #[global_allocator]
// static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

// #[global_allocator]
// static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());

pub struct Locked<A> {
    inner: Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, A> {
        self.inner.lock()
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    // let remainder = addr % align;

    // if remainder == 0 {
    //     addr // addr already aligned
    // } else {
    //     addr - remainder + align
    // }

    // The above is better implemented thus leveraging that align value will always be a power of two coming from GlobalAlloc
    (addr + align - 1) & !(align - 1)
}
