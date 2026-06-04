pub struct Origin {
    pub commit_lsn: i64,
    pub origin: String,
}
pub const TYPE_DESCRIMINATOR: u8 = b'O';

impl crate::utils::DynamicSizeEvent for Origin {
    const MIN_BUFFER_SIZE: usize = std::mem::size_of::<i64>() + 1;
    fn from_buffer(buffer: &[u8]) -> Origin {
        Origin {
            commit_lsn: i64::from_be_bytes(buffer[0..8].try_into().unwrap()),
            origin: std::ffi::CStr::from_bytes_until_nul(&buffer[8..])
                .map(|cstr| cstr.to_string_lossy().into_owned())
                .unwrap_or_default(),
        }
    }
}
