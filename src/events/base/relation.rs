pub trait RelationTrait {
    type Type: crate::utils::DynamicSizeEvent + std::fmt::Debug;
}

#[derive(Debug)]
pub struct RelationWithStreamingEnabled {
    pub transaction_id: i32,
    pub oid: i32,
    pub relation_namespace: String,
    pub name: String,
    pub replica_identity: i8,
    pub columns: Vec<super::relation_column::RelationColumn>,
}

impl crate::utils::DynamicSizeEvent for RelationWithStreamingEnabled {
    const MIN_BUFFER_SIZE: usize = std::mem::size_of::<i32>() * 2
        + 1
        + 1
        + std::mem::size_of::<i8>()
        + std::mem::size_of::<i16>();

    fn from_buffer(buffer: &[u8]) -> RelationWithStreamingEnabled {
        let transaction_id =
            i32::from_be_bytes(buffer[0..4].try_into().unwrap());
        let oid = i32::from_be_bytes(buffer[4..8].try_into().unwrap());
        let relation_namespace =
            std::ffi::CStr::from_bytes_until_nul(&buffer[8..])
                .map(|cstr| cstr.to_string_lossy().into_owned())
                .unwrap_or_default();
        let after_namespace_index = 8 + 1 + relation_namespace.len();
        let name = std::ffi::CStr::from_bytes_until_nul(
            &buffer[after_namespace_index..],
        )
        .map(|cstr| cstr.to_string_lossy().into_owned())
        .unwrap_or_default();
        let after_name_index = after_namespace_index + 1 + name.len();
        let replica_identity = buffer[after_name_index] as i8;
        let column_count = i16::from_be_bytes(
            buffer[after_name_index + 1..after_name_index + 3]
                .try_into()
                .unwrap(),
        );

        Self {
            transaction_id,
            oid,
            relation_namespace,
            name,
            replica_identity,
            columns: super::relation_column::parse_relation_columns(
                column_count,
                &buffer[after_name_index + 2..],
            ),
        }
    }
}

#[derive(Debug)]
pub struct RelationWithoutStreamingEnabled {
    pub oid: i32,
    pub relation_namespace: String,
    pub name: String,
    pub replica_identity: i8,
    pub columns: Vec<super::relation_column::RelationColumn>,
}

impl crate::utils::DynamicSizeEvent for RelationWithoutStreamingEnabled {
    const MIN_BUFFER_SIZE: usize = std::mem::size_of::<i32>()
        + 1
        + 1
        + std::mem::size_of::<i8>()
        + std::mem::size_of::<i16>();

    fn from_buffer(buffer: &[u8]) -> RelationWithoutStreamingEnabled {
        let oid = i32::from_be_bytes(buffer[0..4].try_into().unwrap());
        let relation_namespace =
            std::ffi::CStr::from_bytes_until_nul(&buffer[4..])
                .map(|cstr| cstr.to_string_lossy().into_owned())
                .unwrap_or_default();
        let after_namespace_index = 4 + 1 + relation_namespace.len();
        let name = std::ffi::CStr::from_bytes_until_nul(
            &buffer[after_namespace_index..],
        )
        .map(|cstr| cstr.to_string_lossy().into_owned())
        .unwrap_or_default();
        let after_name_index = after_namespace_index + 1 + name.len();
        let replica_identity = buffer[after_name_index] as i8;
        let column_count = i16::from_be_bytes(
            buffer[after_name_index + 1..after_name_index + 3]
                .try_into()
                .unwrap(),
        );

        Self {
            oid,
            relation_namespace,
            name,
            replica_identity,
            columns: super::relation_column::parse_relation_columns(
                column_count,
                &buffer[after_name_index + 2..],
            ),
        }
    }
}

impl RelationTrait for crate::options::StreamingValueTraitOn {
    type Type = RelationWithStreamingEnabled;
}

impl RelationTrait for crate::options::StreamingValueTraitParallel {
    type Type = RelationWithStreamingEnabled;
}

impl RelationTrait for crate::options::StreamingValueTraitOff {
    type Type = RelationWithoutStreamingEnabled;
}
