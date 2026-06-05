#[derive(Debug)]
#[repr(u8)]
pub enum TwoPhaseCommitEventType {
    BeginPrepare = b'b',
    Prepare = b'P',
    CommitPrepared = b'K',
    RollbackPrepared = b'r',
}

impl TwoPhaseCommitEventType {
    pub fn from_char(c: u8) -> Option<Self> {
        if c == Self::BeginPrepare as u8 {
            Some(Self::BeginPrepare)
        } else if c == Self::Prepare as u8 {
            Some(Self::Prepare)
        } else if c == Self::CommitPrepared as u8 {
            Some(Self::CommitPrepared)
        } else if c == Self::RollbackPrepared as u8 {
            Some(Self::RollbackPrepared)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct BeginPrepare {
    pub lsn: i64,
    pub end_lsn: i64,
    pub timestamp: i64,
    pub transaction_id: i32,
    pub gid: String,
}

impl crate::utils::DynamicSizeEvent for BeginPrepare {
    const MIN_BUFFER_SIZE: usize =
        std::mem::size_of::<i64>() * 3 + std::mem::size_of::<i32>() + 1;
    fn from_buffer(buffer: &[u8]) -> BeginPrepare {
        Self {
            lsn: i64::from_be_bytes(buffer[0..8].try_into().unwrap()),
            end_lsn: i64::from_be_bytes(buffer[8..16].try_into().unwrap()),
            timestamp: i64::from_be_bytes(buffer[16..24].try_into().unwrap()),
            transaction_id: i32::from_be_bytes(
                buffer[24..28].try_into().unwrap(),
            ),
            gid: std::ffi::CStr::from_bytes_until_nul(&buffer[28..])
                .map(|cstr| cstr.to_string_lossy().into_owned())
                .unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct Prepare {
    pub flags: i8,
    pub lsn: i64,
    pub end_lsn: i64,
    pub timestamp: i64,
    pub transaction_id: i32,
    pub gid: String,
}

#[derive(Debug)]
pub struct CommitPrepared {
    pub flags: i8,
    pub lsn: i64,
    pub end_lsn: i64,
    pub timestamp: i64,
    pub transaction_id: i32,
    pub gid: String,
}

#[derive(Debug)]
pub struct RollbackPrepared {
    pub flags: i8,
    pub lsn: i64,
    pub end_lsn: i64,
    pub prepare_timestamp: i64,
    pub rollback_timestamp: i64,
    pub transaction_id: i32,
    pub gid: String,
}

impl crate::utils::DynamicSizeEvent for Prepare {
    const MIN_BUFFER_SIZE: usize = 1 + 8 + 8 + 8 + 4 + 1; // 30 bytes

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

impl crate::utils::DynamicSizeEvent for CommitPrepared {
    const MIN_BUFFER_SIZE: usize = 1 + 8 + 8 + 8 + 4 + 1; // 30 bytes

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

impl crate::utils::DynamicSizeEvent for RollbackPrepared {
    const MIN_BUFFER_SIZE: usize = 1 + 8 + 8 + 8 + 8 + 4 + 1; // 38 bytes

    fn from_buffer(buffer: &[u8]) -> Self {
        let flags = buffer[0] as i8;
        let lsn = i64::from_be_bytes(buffer[1..9].try_into().unwrap());
        let end_lsn = i64::from_be_bytes(buffer[9..17].try_into().unwrap());
        let prepare_timestamp =
            i64::from_be_bytes(buffer[17..25].try_into().unwrap());
        let rollback_timestamp =
            i64::from_be_bytes(buffer[25..33].try_into().unwrap());
        let transaction_id =
            i32::from_be_bytes(buffer[33..37].try_into().unwrap());

        let gid = std::ffi::CStr::from_bytes_until_nul(&buffer[37..])
            .map(|cstr| cstr.to_string_lossy().into_owned())
            .unwrap_or_default();

        Self {
            flags,
            lsn,
            end_lsn,
            prepare_timestamp,
            rollback_timestamp,
            transaction_id,
            gid,
        }
    }
}

#[derive(Debug)]
pub enum TwoPhaseCommitEvent {
    BeginPrepare(BeginPrepare),
    Prepare(Prepare),
    CommitPrepared(CommitPrepared),
    RollbackPrepared(RollbackPrepared),
}

pub fn parse_two_phase_commit_event(
    event_type: &TwoPhaseCommitEventType,
    buffer: &[u8],
) -> Result<TwoPhaseCommitEvent, String> {
    match event_type {
        TwoPhaseCommitEventType::BeginPrepare => {
            crate::utils::parse_dynamic_size_event::<BeginPrepare>(buffer)
                .map(|value| TwoPhaseCommitEvent::BeginPrepare(value))
        }
        TwoPhaseCommitEventType::Prepare => {
            crate::utils::parse_dynamic_size_event::<Prepare>(buffer)
                .map(|value| TwoPhaseCommitEvent::Prepare(value))
        }
        TwoPhaseCommitEventType::CommitPrepared => {
            crate::utils::parse_dynamic_size_event::<CommitPrepared>(buffer)
                .map(|value| TwoPhaseCommitEvent::CommitPrepared(value))
        }
        TwoPhaseCommitEventType::RollbackPrepared => {
            crate::utils::parse_dynamic_size_event::<RollbackPrepared>(buffer)
                .map(|value| TwoPhaseCommitEvent::RollbackPrepared(value))
        }
    }
}
