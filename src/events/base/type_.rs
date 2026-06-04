pub trait TypeTrait: crate::options::StreamingValueTrait {
    type Type: crate::utils::DynamicSizeEvent;
}

pub struct TypeWithStreamingEnabled {
    pub transaction_id: i32,
    pub oid: i32,
    pub type_namespace: String,
    pub name: String,
}

impl crate::utils::DynamicSizeEvent for TypeWithStreamingEnabled {
    const MIN_BUFFER_SIZE: usize = std::mem::size_of::<i32>() * 2 + 1 + 1;

    fn from_buffer(buffer: &[u8]) -> TypeWithStreamingEnabled {
        let transaction_id =
            i32::from_be_bytes(buffer[0..4].try_into().unwrap());
        let oid = i32::from_be_bytes(buffer[4..8].try_into().unwrap());

        let namespace_slice = &buffer[8..];
        let type_namespace =
            std::ffi::CStr::from_bytes_until_nul(namespace_slice)
                .map(|cstr| cstr.to_string_lossy().into_owned())
                .unwrap_or_default();

        let name_slice = &namespace_slice[(type_namespace.len() + 1)..];
        let name = std::ffi::CStr::from_bytes_until_nul(name_slice)
            .map(|cstr| cstr.to_string_lossy().into_owned())
            .unwrap_or_default();

        Self {
            transaction_id: transaction_id,
            oid,
            type_namespace,
            name,
        }
    }
}

impl TypeTrait for crate::options::StreamingValueTraitOn {
    type Type = TypeWithStreamingEnabled;
}

impl TypeTrait for crate::options::StreamingValueTraitParallel {
    type Type = TypeWithStreamingEnabled;
}

pub struct TypeWithoutStreamingEnabled {
    pub oid: i32,
    pub type_namespace: String,
    pub name: String,
}

impl crate::utils::DynamicSizeEvent for TypeWithoutStreamingEnabled {
    const MIN_BUFFER_SIZE: usize = std::mem::size_of::<i32>() + 1 + 1;

    fn from_buffer(buffer: &[u8]) -> TypeWithoutStreamingEnabled {
        let oid = i32::from_be_bytes(buffer[0..4].try_into().unwrap());

        let namespace_slice = &buffer[4..];
        let type_namespace =
            std::ffi::CStr::from_bytes_until_nul(namespace_slice)
                .map(|cstr| cstr.to_string_lossy().into_owned())
                .unwrap_or_default();

        let name_slice = &namespace_slice[(type_namespace.len() + 1)..];
        let name = std::ffi::CStr::from_bytes_until_nul(name_slice)
            .map(|cstr| cstr.to_string_lossy().into_owned())
            .unwrap_or_default();

        Self {
            oid,
            type_namespace,
            name,
        }
    }
}

impl TypeTrait for crate::options::StreamingValueTraitOff {
    type Type = TypeWithoutStreamingEnabled;
}
