use super::align_up;
use core::mem;

struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    const fn new(size: usize) -> Self {
        ListNode { size, next: None }
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

    pub unsafe fn add_free_region(&mut self, addr: usize, size: usize) {

        assert_eq!(align_up(addr, mem::align_of::<ListNode>()), addr);
        assert!(size >= mem::size_of::<ListNode>());

        let mut node = ListNode::new(size);
        node.next = self.head.next.take();

        let node_ptr = addr as *mut ListNode;

        unsafe {
            node_ptr.write(node);
            self.head.next = Some(&mut *node_ptr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization_creates_free_region() {
        // Arrange
        let mut allocator = FreeListAllocator::new(); 

        let mut heap = [0u8; 1024];
        let heap_start = heap.as_mut_ptr() as usize;
        let heap_size = heap.len();

        // Act
        unsafe {
            allocator.init(heap_start, heap_size);
        }

        let free_region = allocator.head.next.as_ref().unwrap();

        assert_eq!(free_region.size, heap_size);
        assert!(free_region.next.is_none());

    }
}