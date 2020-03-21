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
    _8byte_blocks: MemoryBlockList,
    _16byte_blocks: MemoryBlockList,
    _32byte_blocks: MemoryBlockList,
    _64byte_blocks: MemoryBlockList,
    _128byte_blocks: MemoryBlockList,
    _256byte_blocks: MemoryBlockList,
    _512byte_blocks: MemoryBlockList,
    _1024byte_blocks: MemoryBlockList,
    _2048byte_blocks: MemoryBlockList
}

impl Heap {
    pub unsafe fn new(start_address: usize, size: usize) -> Heap {
        let num_block_lists = 9;
        Heap {
            _8byte_blocks: MemoryBlockList::new(8, size / num_block_lists, start_address),
            _16byte_blocks: MemoryBlockList::new(16, size / num_block_lists, start_address + (size / num_block_lists * 1)),
            _32byte_blocks: MemoryBlockList::new(32, size / num_block_lists, start_address + (size / num_block_lists * 2)),
            _64byte_blocks: MemoryBlockList::new(64, size / num_block_lists, start_address + (size / num_block_lists * 3)),
            _128byte_blocks: MemoryBlockList::new(128, size / num_block_lists, start_address + (size / num_block_lists * 4)),
            _256byte_blocks: MemoryBlockList::new(256, size / num_block_lists, start_address + (size / num_block_lists * 5)),
            _512byte_blocks: MemoryBlockList::new(512, size / num_block_lists, start_address + (size / num_block_lists * 6)),
            _1024byte_blocks: MemoryBlockList::new(1024, size / num_block_lists, start_address + (size / num_block_lists * 7)),
            _2048byte_blocks: MemoryBlockList::new(2048, size / num_block_lists, start_address + (size / num_block_lists * 8)),
        }
    }
}

impl Heap {
    unsafe fn alloc(&mut self, _layout: Layout) -> *mut u8 {
        let size = _layout.size();
        let mut block: Option<*mut u8> = None; 

        if size <= 8 {
            block = self._8byte_blocks.alloc();
        } else if size <= 16 {
            block =  self._16byte_blocks.alloc();
        } else if size <=  32 {
            block = self._32byte_blocks.alloc();
        } else if size <= 64 {
            block = self._64byte_blocks.alloc();
        } else if size <= 128 {
            block = self._128byte_blocks.alloc();
        } else if size <= 256 {
            block =  self._256byte_blocks.alloc();
        } else if size <= 512 {
            block = self._512byte_blocks.alloc();
        } else if size <= 1024 {
            block = self._1024byte_blocks.alloc();
        } else if size <= 2048 {
            block = self._2048byte_blocks.alloc();
        }

        if let Some(blk) = block {
            blk
        } else {
            null_mut()
        }
    }

    unsafe fn dealloc(&mut self, _ptr: *mut u8, _layout: Layout) {
        let size = _layout.size();

        if size <= 8 {
            self._8byte_blocks.dealloc(_ptr);
        } else if size <= 16 {
            self._16byte_blocks.dealloc(_ptr);
        } else if size <=  32 {
            self._32byte_blocks.dealloc(_ptr);
        } else if size <= 64 {
            self._64byte_blocks.dealloc(_ptr);
        } else if size <= 128 {
            self._128byte_blocks.dealloc(_ptr);
        } else if size <= 256 {
            self._256byte_blocks.dealloc(_ptr);
        } else if size <= 512 {
            self._512byte_blocks.dealloc(_ptr);
        } else if size <= 1024 {
            self._1024byte_blocks.dealloc(_ptr);
        } else if size <= 2048 {
            self._2048byte_blocks.dealloc(_ptr);
        }
    }

    pub const fn empty() -> Heap {
        Heap {
            _8byte_blocks: MemoryBlockList::empty(),
            _16byte_blocks: MemoryBlockList::empty(),
            _32byte_blocks: MemoryBlockList::empty(),
            _64byte_blocks: MemoryBlockList::empty(),
            _128byte_blocks: MemoryBlockList::empty(),
            _256byte_blocks: MemoryBlockList::empty(),
            _512byte_blocks: MemoryBlockList::empty(),
            _1024byte_blocks: MemoryBlockList::empty(),
            _2048byte_blocks: MemoryBlockList::empty(),
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
            let addr = heap.alloc(layout);
            //crate::serial_println!("Allocated: {:?}, layout: {:?}", addr, layout);
            addr
        } else {
            null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if let Some(ref mut heap) = *self.0.lock() {
            //crate::serial_println!("Deallocate: {:?}, layout: {:?}", ptr, layout);
            heap.dealloc(ptr, layout)
        } 
    }
}
