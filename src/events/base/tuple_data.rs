pub struct PGNull;
pub struct PGUnchangedToastedValue;

pub trait PGValue: crate::options::BinaryValueTrait {
    type Type;
    fn parse(c: u8, value_buffer: &[u8]) -> (Self::Type, usize);
}

impl PGValue for crate::options::BinaryValueTraitOn {
    type Type = Vec<u8>;
    fn parse(c: u8, value_buffer: &[u8]) -> (Self::Type, usize) {
        assert!(c == b'b');
        (value_buffer.to_vec(), 5 + value_buffer.len())
    }
}

impl PGValue for crate::options::BinaryValueTraitOff {
    type Type = String;

    fn parse(c: u8, value_buffer: &[u8]) -> (Self::Type, usize) {
        assert!(c == b't');
        (
            String::from_utf8_lossy(value_buffer).to_string(),
            5 + value_buffer.len(),
        )
    }
}

pub enum TupleDataColumn<T: PGValue> {
    PGNull(PGNull),
    PGUnchangedToastedValue(PGUnchangedToastedValue),
    Value(T::Type),
}

pub type TupleData<Binary> = Vec<TupleDataColumn<Binary>>;

pub fn parse_tuple_column<Binary: PGValue>(
    buffer: &[u8],
) -> (TupleDataColumn<Binary>, usize) {
    let c = buffer.first().unwrap();
    match c {
        b'n' => return (TupleDataColumn::PGNull(PGNull {}), 1),
        b'u' => {
            return (
                TupleDataColumn::PGUnchangedToastedValue(
                    PGUnchangedToastedValue {},
                ),
                1,
            );
        }
        _ => {}
    };
    let value_size = i64::from_be_bytes(buffer[1..4].try_into().unwrap());
    let value_buffer = &buffer[5..value_size as usize];
    let (value, read_bytes) = Binary::parse(*c, value_buffer);
    (TupleDataColumn::Value(value), read_bytes)
}

pub fn parse_tuple_data<Binary: PGValue>(
    buffer: &[u8],
) -> (TupleData<Binary>, usize) {
    let mut data = TupleData::<Binary>::new();
    let column_size = i16::from_be_bytes(buffer[0..2].try_into().unwrap());
    let mut buffer_position = 2;
    for _ in 0..column_size {
        let (column, read_bytes) =
            parse_tuple_column::<Binary>(&buffer[0..buffer_position]);
        data.push(column);
        buffer_position += read_bytes;
    }
    (data, buffer_position)
}

pub type OldTupleData<Binary> = TupleData<Binary>;
pub type PrimaryKeyTupleData<Binary> = TupleData<Binary>;

pub enum OldDataOrPrimaryKeyTupleData<Binary: PGValue> {
    OldTupleData(OldTupleData<Binary>),
    PrimaryKeyTupleData(PrimaryKeyTupleData<Binary>),
}

pub fn parse_old_data_or_primary_key<Binary: PGValue>(
    buffer: &[u8],
) -> Option<(OldDataOrPrimaryKeyTupleData<Binary>, usize)> {
    if buffer.len() == 0 {
        return None;
    };
    let c = buffer.first().unwrap();
    match c {
        b'K' => {
            let (tuple_data, read_bytes) =
                parse_tuple_data::<Binary>(&buffer[0..1]);
            return Some((
                OldDataOrPrimaryKeyTupleData::<Binary>::PrimaryKeyTupleData(
                    tuple_data,
                ),
                read_bytes + 1,
            ));
        }
        b'O' => {
            let (tuple_data, read_bytes) =
                parse_tuple_data::<Binary>(&buffer[0..1]);
            return Some((
                OldDataOrPrimaryKeyTupleData::<Binary>::OldTupleData(
                    tuple_data,
                ),
                read_bytes + 1,
            ));
        }
        _ => {}
    };
    None
}
