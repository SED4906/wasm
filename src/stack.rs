use alloc::vec::Vec;
use core::mem::size_of;
use zerocopy::{AsBytes, FromBytes};

pub struct Stack {
    data: Vec<u8>,
}
impl Stack {
    pub fn push<T: AsBytes>(&mut self, value: T) {
        self.data.append(&mut value.as_bytes().to_vec())
    }
    pub fn pop<T: FromBytes>(&mut self) -> Option<T> {
        let mut bytes = Vec::new();
        for _ in 0..size_of::<T>() {
            bytes.push(self.data.pop()?);
        }
        T::read_from(bytes.as_slice())
    }
}
