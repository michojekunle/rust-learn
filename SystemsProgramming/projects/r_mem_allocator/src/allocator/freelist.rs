use super::{align_up, Locked};
use core::{mem, ptr};
use std::alloc::{GlobalAlloc, Layout};

struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }

    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

pub struct FreeListAllocator {
    head: ListNode,
}

impl FreeListAllocator {
    pub const fn new() -> Self {
        FreeListAllocator {
            head: ListNode::new(0),
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.add_free_region(heap_start, heap_size);
    }

    pub fn alloc_from_region(region: &ListNode, size: usize, align: usize) -> Result<usize, ()> {
        let alloc_start = align_up(region.start_addr(), align);
        let alloc_end = alloc_start.checked_add(size).ok_or(())?;

        if alloc_end > region.end_addr() {
            return Err(());
        }

        let excess_size = region.end_addr() - alloc_end;
        if excess_size > 0 && excess_size < mem::size_of::<ListNode>() {
            return Err(());
        }

        Ok(alloc_start)
    }

    pub fn find_region(
        &mut self,
        size: usize,
        align: usize,
    ) -> Option<(&'static mut ListNode, usize)> {
        let mut current = &mut self.head;

        while let Some(ref mut region) = current.next {
            if let Ok(alloc_start) = Self::alloc_from_region(region, size, align) {
                let next = region.next.take();
                let ret = Some((current.next.take().unwrap(), alloc_start));
                current.next = next;
                return ret;
            } else {
                current = current.next.as_mut().unwrap();
            }
        }

        None
    }

    pub unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        assert_eq!(align_up(addr, mem::align_of::<ListNode>()), addr);
        assert!(size >= mem::size_of::<ListNode>());

        let mut current = &mut self.head;

        while let Some(ref region) = current.next {
            if region.start_addr() >= addr {
                break;
            }

            current = current.next.as_mut().unwrap()
        }

        let mut node = ListNode::new(size);
        node.next = current.next.take();

        let node_ptr = addr as *mut ListNode;

        unsafe {
            node_ptr.write(node);
            current.next = Some(&mut *node_ptr)
        }
    }

    fn size_align(layout: Layout) -> (usize, usize) {
        let layout = layout
            .align_to(mem::align_of::<ListNode>())
            .expect("adjust alignment faiiled")
            .pad_to_align();
        let size = layout.size().max(mem::size_of::<ListNode>());
        (size, layout.align())
    }

    // this is to test the alloc function in my basic tests without having to test using the globalalloc
    unsafe fn allocate(allocator: &mut FreeListAllocator, layout: Layout) -> *mut u8 {
        let (size, align) = FreeListAllocator::size_align(layout);

        if let Some((region, alloc_start)) = allocator.find_region(size, align) {
            let alloc_end = alloc_start.checked_add(size).unwrap();
            let excess_size = region.end_addr() - alloc_end;

            if excess_size > 0 {
                allocator.add_free_region(alloc_end, excess_size);
            }

            alloc_start as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn deallocate(allocator: &mut FreeListAllocator, ptr: *mut u8, layout: Layout) {
        // perform layout adjustments
        let (size, _) = FreeListAllocator::size_align(layout);

        unsafe {
            allocator.add_free_region(ptr as usize, size);
            allocator.coalesce();
        }
    }

    unsafe fn coalesce(&mut self) {
        let mut current = self.head.next.as_mut().unwrap();

        loop {
            let should_merge = current
                .next
                .as_ref()
                .map(|region| current.end_addr() == region.start_addr())
                .unwrap_or(false);

            if should_merge {
                let mut region = current.next.take().unwrap();
                current.size += region.size;
                current.next = region.next.take();
            } else {
                match current.next.as_mut() {
                    Some(next) => current = next,
                    None => break,
                }
            }
        }
    }
}

unsafe impl GlobalAlloc for Locked<FreeListAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let (size, align) = FreeListAllocator::size_align(layout);

        let mut allocator = self.lock();

        if let Some((region, alloc_start)) = allocator.find_region(size, align) {
            let alloc_end = alloc_start.checked_add(size).expect("overflow");
            let excess_size = region.end_addr() - alloc_end;

            if excess_size > 0 {
                unsafe {
                    allocator.add_free_region(alloc_end, excess_size);
                }
            }
            alloc_start as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // perform layout adjustments
        let (size, _) = FreeListAllocator::size_align(layout);

        unsafe {
            let mut allocator = self.lock();

            allocator.add_free_region(ptr as usize, size);
            allocator.coalesce();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create an allocator backed by a local heap.
    fn setup_allocator() -> (FreeListAllocator, [u8; 1024]) {
        let mut allocator = FreeListAllocator::new();
        let mut heap = [0u8; 1024];

        let heap_start = heap.as_mut_ptr() as usize;
        let heap_size = heap.len();

        unsafe {
            allocator.init(heap_start, heap_size);
        }

        (allocator, heap)
    }

    #[test]
    fn test_initialization_creates_free_region() {
        let (allocator, _heap) = setup_allocator();

        let free_region = allocator.head.next.as_ref().unwrap();

        assert_eq!(free_region.size, 1024);
        assert!(free_region.next.is_none());
    }

    #[test]
    fn test_allocates_memory_from_free_region() {
        let (mut allocator, _heap) = setup_allocator();

        let layout = Layout::from_size_align(100, 8).unwrap();
        let (size, align) = unsafe { FreeListAllocator::size_align(layout) };

        let (region, alloc_start) = allocator
            .find_region(size, align)
            .expect("allocator should find a suitable region");

        assert_eq!(alloc_start % align, 0);
        assert!(region.size >= size);
    }

    #[test]
    fn test_allocation_splits_free_region() {
        let (mut allocator, _heap) = setup_allocator();

        let layout = Layout::from_size_align(100, 8).unwrap();
        let (size, align) = unsafe { FreeListAllocator::size_align(layout) };

        let (region, alloc_start) = allocator
            .find_region(size, align)
            .expect("allocator should find a suitable region");

        let alloc_end = alloc_start + size;
        let excess_size = region.end_addr() - alloc_end;

        assert!(excess_size > 0);

        unsafe {
            allocator.add_free_region(alloc_end, excess_size);
        }

        let remaining_region = allocator.head.next.as_ref().unwrap();

        assert_eq!(remaining_region.start_addr(), alloc_end);
        assert_eq!(remaining_region.size, excess_size);
    }

    #[test]
    fn test_multiple_allocations_do_not_overlap() {
        let (mut allocator, _heap) = setup_allocator();

        let layout = Layout::from_size_align(100, 8).unwrap();

        let first = unsafe { FreeListAllocator::allocate(&mut allocator, layout) };
        let second = unsafe { FreeListAllocator::allocate(&mut allocator, layout) };

        assert!(!first.is_null());
        assert!(!second.is_null());

        let first_start = first as usize;
        let second_start = second as usize;

        let size = unsafe { FreeListAllocator::size_align(layout).0 };

        let first_end = first_start + size;
        let second_end = second_start + size;

        assert!(
            second_start >= first_end || first_start >= second_end,
            "allocations overlap"
        );
    }

    #[test]
    fn test_out_of_memory_returns_none() {
        let (mut allocator, _heap) = setup_allocator();

        let layout = Layout::from_size_align(2048, 8).unwrap();
        let (size, align) = unsafe { FreeListAllocator::size_align(layout) };

        let result = allocator.find_region(size, align);

        assert!(result.is_none());
    }

    #[test]
    fn test_large_allocation() {
        let (mut allocator, _heap) = setup_allocator();

        let layout = Layout::from_size_align(900, 8).unwrap();
        let (size, align) = unsafe { FreeListAllocator::size_align(layout) };

        let result = allocator.find_region(size, align);

        assert!(result.is_some());
    }

    #[test]
    fn test_freed_memory_can_be_reused() {
        let (mut allocator, _heap) = setup_allocator();

        let layout = Layout::from_size_align(100, 8).unwrap();

        let first = unsafe { FreeListAllocator::allocate(&mut allocator, layout) };

        assert!(!first.is_null());

        unsafe {
            FreeListAllocator::deallocate(&mut allocator, first, layout);
        }

        let second = unsafe { FreeListAllocator::allocate(&mut allocator, layout) };

        assert!(!second.is_null());
        assert_eq!(second, first);
    }

    #[test]
    fn test_dealloc() {
        let (mut allocator, heap) = setup_allocator();

        let layout = Layout::from_size_align(100, 8).unwrap();
        let (size, align) = unsafe { FreeListAllocator::size_align(layout) };

        let (_, alloc_start) = allocator.find_region(size, align).unwrap();

        unsafe {
            FreeListAllocator::deallocate(&mut allocator, alloc_start as *mut u8, layout);
        }

        let region = allocator.head.next.as_ref().unwrap();

        assert_eq!(region.start_addr(), alloc_start);
        assert_eq!(region.size, size);
        assert!(region.next.is_none());

        drop(heap);
    }

    #[test]
    fn test_dealloc_coalesces_two_regions() {
        let (mut allocator, heap) = setup_allocator();

        let layout = Layout::from_size_align(100, 8).unwrap();

        let first = unsafe { FreeListAllocator::allocate(&mut allocator, layout) };

        let second = unsafe { FreeListAllocator::allocate(&mut allocator, layout) };
        unsafe {
            FreeListAllocator::deallocate(&mut allocator, second as *mut u8, layout);

            FreeListAllocator::deallocate(&mut allocator, first as *mut u8, layout);
        }

        let region = allocator.head.next.as_ref().unwrap();

        assert_eq!(region.start_addr(), first as usize);
        assert_eq!(region.size, heap.len());
        assert!(region.next.is_none());

        let _ = heap;
    }

    #[test]
    fn test_dealloc_coalesces_three_regions() {
        let (mut allocator, heap) = setup_allocator();

        let layout = Layout::from_size_align(100, 8).unwrap();

        let first = unsafe { FreeListAllocator::allocate(&mut allocator, layout) };
        let second = unsafe { FreeListAllocator::allocate(&mut allocator, layout) };
        let third = unsafe { FreeListAllocator::allocate(&mut allocator, layout) };

        unsafe {
            FreeListAllocator::deallocate(&mut allocator, first as *mut u8, layout);

            FreeListAllocator::deallocate(&mut allocator, second as *mut u8, layout);

            FreeListAllocator::deallocate(&mut allocator, third as *mut u8, layout);
        }

        let region = allocator.head.next.as_ref().unwrap();

        assert_eq!(region.start_addr(), first as usize);
        assert_eq!(region.size, heap.len());
        assert!(region.next.is_none());

        let _ = heap;
    }

    #[test]
    fn test_dealloc_does_not_coalesce_non_adjacent_regions() {
        let (mut allocator, _heap) = setup_allocator();

        let layout = Layout::from_size_align(100, 8).unwrap();

        let first = unsafe { FreeListAllocator::allocate(&mut allocator, layout) };

        let _second = unsafe { FreeListAllocator::allocate(&mut allocator, layout) };

        let third = unsafe { FreeListAllocator::allocate(&mut allocator, layout) };

        unsafe {
            FreeListAllocator::deallocate(&mut allocator, first, layout);
            FreeListAllocator::deallocate(&mut allocator, third, layout);
        }

        let first_region = allocator.head.next.as_ref().unwrap();
        let second_region = first_region.next.as_ref().unwrap();

        let (size, _) = unsafe { FreeListAllocator::size_align(layout) };

        assert_eq!(first_region.start_addr(), first as usize);
        assert_eq!(first_region.size, size);

        assert_eq!(second_region.start_addr(), third as usize);
        assert!(second_region.size > size);
        assert!(second_region.next.is_none());
    }

    #[test]
    fn test_dealloc_coalesces_in_any_order() {
        let (mut allocator, heap) = setup_allocator();

        let layout = Layout::from_size_align(100, 8).unwrap();

        let first = unsafe { FreeListAllocator::allocate(&mut allocator, layout) };

        let second = unsafe { FreeListAllocator::allocate(&mut allocator, layout) };

        let third = unsafe { FreeListAllocator::allocate(&mut allocator, layout) };

        unsafe {
            FreeListAllocator::deallocate(&mut allocator, second, layout);
            FreeListAllocator::deallocate(&mut allocator, first, layout);
            FreeListAllocator::deallocate(&mut allocator, third, layout);
        }

        let region = allocator.head.next.as_ref().unwrap();

        assert_eq!(region.start_addr(), first as usize);
        assert_eq!(region.size, heap.len());
        assert!(region.next.is_none());
    }
}
