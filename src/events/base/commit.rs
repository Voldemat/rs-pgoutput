pub struct Commit {
    pub flags: i8,
    pub lsn: i64,
    pub end_lsn: i64,
    pub timestamp: i64,
}

impl crate::utils::StaticSizeEvent for Commit {
    const BUFFER_SIZE: usize =
        std::mem::size_of::<i8>() + std::mem::size_of::<i64>() * 3;
    fn from_buffer(buffer: &[u8]) -> Self {
        Self {
            flags: buffer[0] as i8,
            lsn: i64::from_be_bytes(buffer[1..9].try_into().unwrap()),
            end_lsn: i64::from_be_bytes(buffer[9..17].try_into().unwrap()),
            timestamp: i64::from_be_bytes(buffer[17..25].try_into().unwrap()),
        }
    }
}
