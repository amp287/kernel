#[macro_export]
macro_rules! set_bit {
    ($data:expr, $bit_num:expr, $set_to:expr) => {
        $data &= !(1 << $bit_num);
        $data |= $set_to << $bit_num;
    }
}

#[macro_export]
macro_rules! set_bits {
    ($data:expr, $start_bit:expr, $set_to:expr, $mask:expr) => {
        $data &= !($mask << $start_bit);
        $data |= $set_to << $start_bit;
    }
}

#[macro_export]
macro_rules! get_bits {
    ($data:expr, $start_bit:expr, $mask:expr) => {
        {
            let mut copy = $data;
            copy = copy >> $start_bit;
            copy &= $mask;
            copy
        }
    }
}