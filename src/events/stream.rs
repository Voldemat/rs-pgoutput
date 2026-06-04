#[repr(u8)]
pub enum StreamEventType {
    StreamStart = b'S',
    StreamStop = b'E',
    StreamCommit = b'c',
    StreamAbort = b'A',
}

impl StreamEventType {
    pub fn from_char(c: u8) -> Option<Self> {
        match c {
            b'S' => Some(Self::StreamStart),
            b'E' => Some(Self::StreamStop),
            b'c' => Some(Self::StreamCommit),
            b'A' => Some(Self::StreamAbort),
            _ => None,
        }
    }
}

pub struct StreamStart {
    pub transaction_id: i32,
    pub flags: i8,
}

impl crate::utils::StaticSizeEvent for StreamStart {
    const BUFFER_SIZE: usize =
        std::mem::size_of::<i32>() + std::mem::size_of::<i8>();
    fn from_buffer(buffer: &[u8]) -> Self {
        Self {
            transaction_id: i32::from_be_bytes(
                buffer[0..4].try_into().unwrap(),
            ),
            flags: buffer[4] as i8,
        }
    }
}

pub struct StreamStop;

pub struct StreamCommit {
    pub transaction_id: i32,
    pub flags: i8,
    pub lsn: i64,
    pub end_lsn: i64,
    pub timestamp: i64,
}

impl crate::utils::StaticSizeEvent for StreamCommit {
    const BUFFER_SIZE: usize = 4 + 1 + 8 + 8 + 8;
    fn from_buffer(buffer: &[u8]) -> Self {
        Self {
            transaction_id: i32::from_be_bytes(
                buffer[0..4].try_into().unwrap(),
            ),
            flags: buffer[4] as i8,
            lsn: i64::from_be_bytes(buffer[5..13].try_into().unwrap()),
            end_lsn: i64::from_be_bytes(buffer[13..21].try_into().unwrap()),
            timestamp: i64::from_be_bytes(buffer[21..29].try_into().unwrap()),
        }
    }
}

pub trait StreamAbortTrait {
    type Type: crate::utils::StaticSizeEvent;
}

pub struct StreamAbortWithoutParallel {
    pub transaction_id: i32,
    pub sub_transaction_id: i32,
}

impl crate::utils::StaticSizeEvent for StreamAbortWithoutParallel {
    const BUFFER_SIZE: usize = std::mem::size_of::<i32>() * 2;
    fn from_buffer(buffer: &[u8]) -> Self {
        Self {
            transaction_id: i32::from_be_bytes(
                buffer[0..4].try_into().unwrap(),
            ),
            sub_transaction_id: i32::from_be_bytes(
                buffer[4..8].try_into().unwrap(),
            ),
        }
    }
}

pub struct StreamAbortWithParallel {
    pub transaction_id: i32,
    pub sub_transaction_id: i32,
    pub lsn: i64,
    pub timestamp: i64,
}

impl crate::utils::StaticSizeEvent for StreamAbortWithParallel {
    const BUFFER_SIZE: usize =
        std::mem::size_of::<i32>() * 2 + std::mem::size_of::<i64>();
    fn from_buffer(buffer: &[u8]) -> Self {
        Self {
            transaction_id: i32::from_be_bytes(
                buffer[0..4].try_into().unwrap(),
            ),
            sub_transaction_id: i32::from_be_bytes(
                buffer[4..8].try_into().unwrap(),
            ),
            lsn: i64::from_be_bytes(buffer[8..16].try_into().unwrap()),
            timestamp: i64::from_be_bytes(buffer[16..24].try_into().unwrap()),
        }
    }
}

impl StreamAbortTrait for crate::options::StreamingValueTraitOn {
    type Type = StreamAbortWithoutParallel;
}

impl StreamAbortTrait for crate::options::StreamingValueTraitParallel {
    type Type = StreamAbortWithParallel;
}

impl StreamAbortTrait for crate::options::StreamingValueTraitOff {
    type Type = StreamAbortWithoutParallel;
}

pub enum StreamEvent<Streaming: StreamAbortTrait> {
    Start(StreamStart),
    Stop(StreamStop),
    Commit(StreamCommit),
    Abort(Streaming::Type),
}

pub fn parse_streaming_event<Streaming: StreamAbortTrait>(
    event_type: &StreamEventType,
    buffer: &[u8],
) -> Result<StreamEvent<Streaming>, String> {
    match event_type {
        StreamEventType::StreamStart => {
            crate::utils::parse_static_size_event::<StreamStart>(buffer)
                .map(|value| StreamEvent::Start(value))
        }
        StreamEventType::StreamStop => Ok(StreamEvent::Stop(StreamStop)),
        StreamEventType::StreamCommit => {
            crate::utils::parse_static_size_event::<StreamCommit>(buffer)
                .map(|value| StreamEvent::Commit(value))
        }
        StreamEventType::StreamAbort => {
            crate::utils::parse_static_size_event::<Streaming::Type>(buffer)
                .map(|value| StreamEvent::Abort(value))
        }
    }
}
