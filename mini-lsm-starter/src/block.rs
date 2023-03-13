mod builder;
mod iterator;

pub use builder::BlockBuilder;
use bytes::{Buf, BufMut, Bytes};
pub use iterator::BlockIterator;


const SZ_U16: usize = 2;
const SZ_U8: usize = 1;

/// A block is the smallest unit of read and caching in LSM tree. It is a collection of sorted
/// key-value pairs.
pub struct Block {
    data: Vec<u8>,
    offsets: Vec<u16>,
}

impl Block {
    pub fn encode(&self) -> Bytes {
        let mut out = self.data.clone();
        for offset in &self.offsets {
            out.put_u16_le(*offset);
        }
        out.put_u16_le(self.offsets.len() as u16);

        Bytes::from(out)
    }

    pub fn decode(data: &[u8]) -> Self {
        let mut offsets_length_byes = &data[data.len() - SZ_U16..];
        let offset_length = offsets_length_byes.get_u16_le() as usize;

        let offset_end = data.len() - SZ_U16;
        let offset_start = data.len() - SZ_U16 - offset_length * SZ_U16;

        let offsets_bytes = &data[offset_start..offset_end];

        let offsets_vec: Vec<u16> = Vec::from_iter(offsets_bytes.iter())
            .chunks_exact(SZ_U16)
            .into_iter()
            .map(|a| u16::from_le_bytes([*a[0], *a[1]]))
            .collect();


        Block {
            data: data[0..offset_start].to_vec(),
            offsets: offsets_vec,
        }
    }
}

#[cfg(test)]
mod tests;
