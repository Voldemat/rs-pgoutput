pub struct StreamPrepare {
    pub flags: i8,
    pub lsn: i64,
    pub end_lsn: i64,
    pub timestamp: i64,
    pub transaction_id: i32,
    pub gid: String,
}

pub const TYPE_DESCRIMINATOR: u8 = b'p';

impl crate::utils::DynamicSizeEvent for StreamPrepare {
    const MIN_BUFFER_SIZE: usize = std::mem::size_of::<i8>()
        + std::mem::size_of::<i64>() * 3
        + std::mem::size_of::<i32>()
        + 1;

    fn from_buffer(buffer: &[u8]) -> Self {
        let flags = buffer[0] as i8;
        let lsn = i64::from_be_bytes(buffer[1..9].try_into().unwrap());
        let end_lsn = i64::from_be_bytes(buffer[9..17].try_into().unwrap());
        let timestamp = i64::from_be_bytes(buffer[17..25].try_into().unwrap());
        let transaction_id =
            i32::from_be_bytes(buffer[25..29].try_into().unwrap());

        let gid = std::ffi::CStr::from_bytes_until_nul(&buffer[29..])
            .map(|cstr| cstr.to_string_lossy().into_owned())
            .unwrap_or_default();

        Self {
            flags,
            lsn,
            end_lsn,
            timestamp,
            transaction_id,
            gid,
        }
    }
}
