// MailBox0 at MAILBOX_ADDR GPU -> CPU
// MailBox1 at MAILBOX_ADDR + 0x20 CPU -> GPU

/*use volatile_register::{RW, RO};

static MMIO_BASE = 0x3F000000;
static MAILBOX_ADDR = MMIO_BASE + 0x0000B880;
static MAIL_EMPTY = 0x40000000;
static MAIL_FULL = 0x80000000

#[repr(C, packed)]
struct MailBox { 
    read_write: RW<u32>,
    reserved: RO<[u32, 3]>,
    peek: RO<u32>,
    status: RO<u32>,
    config: RW<u32>
}

#[repr(C, packed, align(16))]
struct MailBoxMessage {
    size: RW<u32>,
    request_response: RW<u32>,
}

pub fn mailbox_read_channel(channel: u8) -> u32 {
    let data: u32;

    unsafe {
        let mb0 = MAILBOX_ADDR as *mut MailBox;
        loop {
            while ((*mb0).status.read() & MAIL_EMPTY != 0) {}

            data = (*mbo).read_write.read();

            if(data & 0xF == channel) {
                return data >> 4;
            }
        }
    }
    

}

// data should only be 28 bits
pub fn mailbox_write_channel(channel: u8, data: u32) {
    let mb1 = MAILBOX_ADDR + 0x20 as *mut MailBox

    while((*mb1).status.read() & MAIL_FULL != 0) {
    }

    (*mb1).read_write.write( (data << 4) | channel);

}*/