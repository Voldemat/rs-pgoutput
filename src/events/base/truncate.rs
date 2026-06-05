pub trait TruncateTrait: crate::options::StreamingValueTrait {
    type Type: crate::utils::DynamicSizeEvent + std::fmt::Debug;
}

#[derive(Debug)]
pub struct TruncateWithStreamingEnabled {
    pub transaction_id: i32,
    pub flags: i8,
    pub oids: Vec<i32>,
}

impl crate::utils::DynamicSizeEvent for TruncateWithStreamingEnabled {
    const MIN_BUFFER_SIZE: usize = std::mem::size_of::<i32>()
        + std::mem::size_of::<i8>()
        + std::mem::size_of::<i32>();

    fn from_buffer(buffer: &[u8]) -> TruncateWithStreamingEnabled {
        let relations_count =
            i32::from_be_bytes(buffer[4..8].try_into().unwrap()) as usize;
        Self {
            transaction_id: i32::from_be_bytes(
                buffer[0..4].try_into().unwrap(),
            ),
            flags: buffer[8] as i8,
            oids: buffer[9..]
                .chunks_exact(4)
                .take(relations_count)
                .map(|chunk| i32::from_be_bytes(chunk.try_into().unwrap()))
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct TruncateWithoutStreamingEnabled {
    pub flags: i8,
    pub oids: Vec<i32>,
}

impl crate::utils::DynamicSizeEvent for TruncateWithoutStreamingEnabled {
    const MIN_BUFFER_SIZE: usize =
        std::mem::size_of::<i8>() * std::mem::size_of::<i32>();
    fn from_buffer(buffer: &[u8]) -> Self {
        let relations_count =
            i32::from_be_bytes(buffer[0..4].try_into().unwrap()) as usize;
        Self {
            flags: buffer[4] as i8,
            oids: buffer[5..]
                .chunks_exact(4)
                .take(relations_count)
                .map(|chunk| i32::from_be_bytes(chunk.try_into().unwrap()))
                .collect(),
        }
    }
}

impl TruncateTrait for crate::options::StreamingValueTraitOn {
    type Type = TruncateWithStreamingEnabled;
}

impl TruncateTrait for crate::options::StreamingValueTraitParallel {
    type Type = TruncateWithStreamingEnabled;
}

impl TruncateTrait for crate::options::StreamingValueTraitOff {
    type Type = TruncateWithoutStreamingEnabled;
}
