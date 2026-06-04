pub const TYPE_DESCRIMINATOR: u8 = b'M';
pub trait MessageTrait {
    type Type: crate::utils::DynamicSizeEvent;
}

pub struct MessageWithStreamingEnabled {
    pub transaction_id: i32,
    pub flags: i8,
    pub lsn: i64,
    pub prefix: String,
    pub content: Vec<u8>,
}

impl crate::utils::DynamicSizeEvent for MessageWithStreamingEnabled {
    const MIN_BUFFER_SIZE: usize = std::mem::size_of::<i32>()
        + std::mem::size_of::<i8>()
        + std::mem::size_of::<i64>()
        + std::mem::size_of::<String>()
        + std::mem::size_of::<i32>();

    fn from_buffer(buffer: &[u8]) -> MessageWithStreamingEnabled {
        let prefix = std::ffi::CStr::from_bytes_until_nul(&buffer[13..])
            .map(|cstr| cstr.to_string_lossy().into_owned())
            .unwrap_or_default();
        let content_length = i32::from_be_bytes(
            buffer[13 + prefix.len() + 1..13 + prefix.len() + 1 + 4]
                .try_into()
                .unwrap(),
        );
        let content = buffer[13 + prefix.len() + 1 + 4
            ..13 + prefix.len() + 1 + 4 + content_length as usize]
            .to_vec();
        MessageWithStreamingEnabled {
            transaction_id: i32::from_be_bytes(
                buffer[0..4].try_into().unwrap(),
            ),
            flags: buffer[4] as i8,
            lsn: i64::from_be_bytes(buffer[5..13].try_into().unwrap()),
            prefix,
            content,
        }
    }
}

pub struct MessageWithoutStreamingEnabled {
    pub flags: i8,
    pub lsn: i64,
    pub prefix: String,
    pub content: Vec<u8>,
}

impl crate::utils::DynamicSizeEvent for MessageWithoutStreamingEnabled {
    const MIN_BUFFER_SIZE: usize = std::mem::size_of::<i8>()
        + std::mem::size_of::<i64>()
        + std::mem::size_of::<String>()
        + std::mem::size_of::<i32>();

    fn from_buffer(buffer: &[u8]) -> Self {
        let prefix = std::ffi::CStr::from_bytes_until_nul(&buffer[9..])
            .map(|cstr| cstr.to_string_lossy().into_owned())
            .unwrap_or_default();
        let after_prefix_position = 9 + prefix.len() + 1;
        let content_length = i32::from_be_bytes(
            buffer[after_prefix_position..after_prefix_position + 4]
                .try_into()
                .unwrap(),
        );
        let after_content_length_position = after_prefix_position + 4;
        Self {
            flags: buffer[0] as i8,
            lsn: i64::from_be_bytes(buffer[1..9].try_into().unwrap()),
            prefix,
            content: buffer[after_content_length_position
                ..after_content_length_position + content_length as usize]
                .to_vec(),
        }
    }
}

impl MessageTrait for crate::options::StreamingValueTraitOn {
    type Type = MessageWithStreamingEnabled;
}

impl MessageTrait for crate::options::StreamingValueTraitParallel {
    type Type = MessageWithStreamingEnabled;
}

impl MessageTrait for crate::options::StreamingValueTraitOff {
    type Type = MessageWithoutStreamingEnabled;
}
