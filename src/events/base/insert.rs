pub trait InsertTrait<Binary> {
    type Type: crate::utils::DynamicSizeEvent + std::fmt::Debug;
}

#[derive(Debug)]
pub struct InsertWithStreamingEnabled<
    Binary: super::tuple_data::PGValue + std::fmt::Debug,
> {
    pub transaction_id: i32,
    pub oid: i32,
    pub data: super::tuple_data::TupleData<Binary>,
}

impl<Binary: super::tuple_data::PGValue + std::fmt::Debug>
    crate::utils::DynamicSizeEvent for InsertWithStreamingEnabled<Binary>
{
    const MIN_BUFFER_SIZE: usize =
        std::mem::size_of::<i32>() * 2 + std::mem::size_of::<i16>();

    fn from_buffer(buffer: &[u8]) -> InsertWithStreamingEnabled<Binary> {
        InsertWithStreamingEnabled {
            transaction_id: i32::from_be_bytes(
                buffer[0..4].try_into().unwrap(),
            ),
            oid: i32::from_be_bytes(buffer[4..8].try_into().unwrap()),
            data: super::tuple_data::parse_tuple_data::<Binary>(&buffer[9..]).0,
        }
    }
}

#[derive(Debug)]
pub struct InsertWithoutStreamingEnabled<
    Binary: super::tuple_data::PGValue + std::fmt::Debug,
> {
    pub oid: i32,
    pub data: super::tuple_data::TupleData<Binary>,
}

impl<Binary: super::tuple_data::PGValue + std::fmt::Debug>
    crate::utils::DynamicSizeEvent for InsertWithoutStreamingEnabled<Binary>
{
    const MIN_BUFFER_SIZE: usize =
        std::mem::size_of::<i32>() + std::mem::size_of::<i16>();

    fn from_buffer(buffer: &[u8]) -> InsertWithoutStreamingEnabled<Binary> {
        InsertWithoutStreamingEnabled {
            oid: i32::from_be_bytes(buffer[0..4].try_into().unwrap()),
            data: super::tuple_data::parse_tuple_data::<Binary>(&buffer[5..]).0,
        }
    }
}

impl<Binary: super::tuple_data::PGValue + std::fmt::Debug> InsertTrait<Binary>
    for crate::options::StreamingValueTraitOn
{
    type Type = InsertWithStreamingEnabled<Binary>;
}

impl<Binary: super::tuple_data::PGValue + std::fmt::Debug> InsertTrait<Binary>
    for crate::options::StreamingValueTraitParallel
{
    type Type = InsertWithStreamingEnabled<Binary>;
}

impl<Binary: super::tuple_data::PGValue + std::fmt::Debug> InsertTrait<Binary>
    for crate::options::StreamingValueTraitOff
{
    type Type = InsertWithoutStreamingEnabled<Binary>;
}
