pub struct Begin {
    pub final_transaction_lsn: i64,
    pub commit_timestamp: i64,
    pub transaction_id: i32,
}

impl crate::utils::StaticSizeEvent for Begin {
    const BUFFER_SIZE: usize =
        std::mem::size_of::<i64>() * 2 + std::mem::size_of::<i32>();
    fn from_buffer(buffer: &[u8]) -> Self {
        Self {
            final_transaction_lsn: i64::from_be_bytes(
                buffer[0..8].try_into().unwrap(),
            ),
            commit_timestamp: i64::from_be_bytes(
                buffer[8..16].try_into().unwrap(),
            ),
            transaction_id: i32::from_be_bytes(
                buffer[16..20].try_into().unwrap(),
            ),
        }
    }
}
