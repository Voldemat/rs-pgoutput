pub trait DeleteTrait<Binary> {
    type Type: crate::utils::DynamicSizeEvent + std::fmt::Debug;
}

#[derive(Debug)]
pub struct DeleteWithStreamingEnabled<Binary: super::tuple_data::PGValue> {
    pub transaction_id: i32,
    pub oid: i32,
    pub old_data_or_primary_key:
        Option<super::tuple_data::OldDataOrPrimaryKeyTupleData<Binary>>,
}

impl<Binary: super::tuple_data::PGValue> crate::utils::DynamicSizeEvent
    for DeleteWithStreamingEnabled<Binary>
{
    const MIN_BUFFER_SIZE: usize = std::mem::size_of::<i32>() * 2;
    fn from_buffer(buffer: &[u8]) -> DeleteWithStreamingEnabled<Binary> {
        let transaction_id =
            i32::from_be_bytes(buffer[0..4].try_into().unwrap());
        let oid = i32::from_be_bytes(buffer[4..8].try_into().unwrap());
        DeleteWithStreamingEnabled {
            transaction_id: transaction_id,
            oid: oid,
            old_data_or_primary_key:
                super::tuple_data::parse_old_data_or_primary_key::<Binary>(
                    &buffer[8..],
                )
                .map(|(value, _)| value),
        }
    }
}

#[derive(Debug)]
pub struct DeleteWithoutStreamingEnabled<Binary: super::tuple_data::PGValue> {
    pub oid: i32,
    pub old_data_or_primary_key:
        Option<super::tuple_data::OldDataOrPrimaryKeyTupleData<Binary>>,
}

impl<Binary: super::tuple_data::PGValue> crate::utils::DynamicSizeEvent
    for DeleteWithoutStreamingEnabled<Binary>
{
    const MIN_BUFFER_SIZE: usize = std::mem::size_of::<i32>();
    fn from_buffer(buffer: &[u8]) -> DeleteWithoutStreamingEnabled<Binary> {
        DeleteWithoutStreamingEnabled {
            oid: i32::from_be_bytes(buffer[0..4].try_into().unwrap()),
            old_data_or_primary_key:
                super::tuple_data::parse_old_data_or_primary_key::<Binary>(
                    &buffer[4..],
                )
                .map(|(value, _)| value),
        }
    }
}

impl<Binary: super::tuple_data::PGValue> DeleteTrait<Binary>
    for crate::options::StreamingValueTraitOn
{
    type Type = DeleteWithStreamingEnabled<Binary>;
}

impl<Binary: super::tuple_data::PGValue> DeleteTrait<Binary>
    for crate::options::StreamingValueTraitParallel
{
    type Type = DeleteWithStreamingEnabled<Binary>;
}

impl<Binary: super::tuple_data::PGValue> DeleteTrait<Binary>
    for crate::options::StreamingValueTraitOff
{
    type Type = DeleteWithoutStreamingEnabled<Binary>;
}
