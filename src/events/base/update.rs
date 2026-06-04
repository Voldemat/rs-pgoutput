pub trait UpdateTrait<Binary: super::tuple_data::PGValue>:
    crate::options::StreamingValueTrait
{
    type Type: crate::utils::DynamicSizeEvent;
}

pub struct UpdateWithStreamingEnabled<Binary: super::tuple_data::PGValue> {
    pub transaction_id: i32,
    pub oid: i32,
    pub old_data_or_primary_key:
        Option<super::tuple_data::OldDataOrPrimaryKeyTupleData<Binary>>,
    pub data: super::tuple_data::TupleData<Binary>,
}

impl<Binary: super::tuple_data::PGValue> crate::utils::DynamicSizeEvent
    for UpdateWithStreamingEnabled<Binary>
{
    const MIN_BUFFER_SIZE: usize =
        std::mem::size_of::<i32>() * 2 + std::mem::size_of::<i16>();
    fn from_buffer(buffer: &[u8]) -> Self {
        let transaction_id =
            i32::from_be_bytes(buffer[0..4].try_into().unwrap());
        let oid = i32::from_be_bytes(buffer[4..8].try_into().unwrap());
        let (old_data_or_primary_key, read_bytes) =
            super::tuple_data::parse_old_data_or_primary_key::<Binary>(
                &buffer[8..],
            )
            .map(|(value, r)| (Some(value), r))
            .unwrap_or((None, 0));
        UpdateWithStreamingEnabled {
            transaction_id,
            oid: oid,
            old_data_or_primary_key,
            data: super::tuple_data::parse_tuple_data::<Binary>(
                &buffer[9 + read_bytes..],
            )
            .0,
        }
    }
}

pub struct UpdateWithoutStreamingEnabled<Binary: super::tuple_data::PGValue> {
    pub oid: i32,
    pub old_data_or_primary_key:
        Option<super::tuple_data::OldDataOrPrimaryKeyTupleData<Binary>>,
    pub data: super::tuple_data::TupleData<Binary>,
}

impl<Binary: super::tuple_data::PGValue> crate::utils::DynamicSizeEvent
    for UpdateWithoutStreamingEnabled<Binary>
{
    const MIN_BUFFER_SIZE: usize =
        std::mem::size_of::<i32>() + std::mem::size_of::<i16>();
    fn from_buffer(buffer: &[u8]) -> UpdateWithoutStreamingEnabled<Binary> {
        let oid = i32::from_be_bytes(buffer[0..4].try_into().unwrap());
        let (old_data_or_primary_key, read_bytes) =
            super::tuple_data::parse_old_data_or_primary_key::<Binary>(
                &buffer[4..],
            )
            .map(|(value, r)| (Some(value), r))
            .unwrap_or((None, 0));
        UpdateWithoutStreamingEnabled {
            oid: oid,
            old_data_or_primary_key,
            data: super::tuple_data::parse_tuple_data::<Binary>(
                &buffer[5 + read_bytes..],
            )
            .0,
        }
    }
}

impl<Binary: super::tuple_data::PGValue> UpdateTrait<Binary>
    for crate::options::StreamingValueTraitOn
{
    type Type = UpdateWithStreamingEnabled<Binary>;
}

impl<Binary: super::tuple_data::PGValue> UpdateTrait<Binary>
    for crate::options::StreamingValueTraitParallel
{
    type Type = UpdateWithStreamingEnabled<Binary>;
}

impl<Binary: super::tuple_data::PGValue> UpdateTrait<Binary>
    for crate::options::StreamingValueTraitOff
{
    type Type = UpdateWithoutStreamingEnabled<Binary>;
}
