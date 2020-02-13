extern crate alloc;

use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use spin::Mutex;


struct Block {
    next: Option<&'static Block>
}

struct MemoryBlockList {
    block_size: usize,
    start_address: usize,
    number_of_blocks: usize,
    blocks: Option<&'static Block>   
}

impl MemoryBlockList {
    // start_address: start of heap address, will grow down
    unsafe fn new(block_size: usize, slab_size: usize, start_address: usize) ->  MemoryBlockList {
        let number_of_blocks = slab_size / block_size;
        let mut mem_block_list = MemoryBlockList::empty();

        mem_block_list.start_address = start_address;
        mem_block_list.block_size = block_size;

        for i in (0 .. number_of_blocks).rev() {
            let block: *mut Block = (start_address + (i * block_size)) as *mut Block;
            mem_block_list.push(&mut *block);
        }

       mem_block_list
    }

    fn push(&mut self, block: &'static mut Block) {
        block.next = self.blocks.take();
        self.number_of_blocks += 1;
        self.blocks.get_or_insert(block);
    }

    fn pop(&mut self) -> Option<&'static Block> {
        let old_head:Option<&'static Block> = self.blocks.take();
        match old_head {
            Some(block) => {
                self.blocks = block.next;
                self.number_of_blocks -= 1;
                Some(block)
            },
            None => None
        }
    }

    fn alloc(&mut self) -> Option<*mut u8> {
        if self.number_of_blocks == 0 {
            return None;
        }

      self.pop().map(|block| {
           block as *const Block as *mut u8
       })
    }

    fn dealloc(&mut self, ptr: *mut u8) {
        let block = ptr as *mut Block;
        unsafe {self.push(&mut *block);}
    }

    pub const fn empty() -> MemoryBlockList {
        MemoryBlockList { 
            block_size: 0,
            number_of_blocks: 0,
            start_address: 0,
            blocks: None
        }
    }
}

pub struct Heap {
    _64byte_blocks: MemoryBlockList
}

impl Heap {
    pub unsafe fn new(start_address: usize, size: usize) -> Heap {
        Heap {
            _64byte_blocks: MemoryBlockList::new(64, size, start_address),
        }
    }
}

impl Heap {
    unsafe fn alloc(&mut self, _layout: Layout) -> *mut u8 {
        if let Some(block) = self._64byte_blocks.alloc() {
            block
        } else {
            null_mut()
        }
    }

    unsafe fn dealloc(&mut self, _ptr: *mut u8, _layout: Layout) {
       self._64byte_blocks.dealloc(_ptr);
    }

    pub const fn empty() -> Heap {
        Heap {
            _64byte_blocks: MemoryBlockList::empty()
        }
    }
}

pub struct LockedHeap (Mutex<Option<Heap>>);

impl LockedHeap {
    pub unsafe fn init(&mut self, heap_start_addr: usize, size: usize) {
        *self.0.lock() = Some(Heap::new(heap_start_addr, size));
    }

    pub const fn empty() -> LockedHeap {
        LockedHeap(Mutex::new(None))
    }
}

unsafe impl GlobalAlloc for LockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if let Some(ref mut heap) = *self.0.lock() {
            heap.alloc(layout)
        } else {
            null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if let Some(ref mut heap) = *self.0.lock() {
            heap.dealloc(ptr, layout)
        } 
    }
}
