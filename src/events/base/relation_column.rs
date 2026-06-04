pub struct RelationColumn {
    pub flags: i8,
    pub name: String,
    pub oid: i32,
    pub type_modifier: i32,
}

impl RelationColumn {
    const MIN_BUFFER_SIZE: usize =
        std::mem::size_of::<i8>() + 1 + std::mem::size_of::<i32>() * 2;

    fn get_buffer_size(self: &Self) -> usize {
        Self::MIN_BUFFER_SIZE + self.name.len() + 1
    }

    fn from_buffer(buffer: &[u8]) -> RelationColumn {
        let flags = buffer[0] as i8;
        let name_slice = &buffer[1..];
        let name = std::ffi::CStr::from_bytes_until_nul(name_slice)
            .map(|cstr| cstr.to_string_lossy().into_owned())
            .unwrap_or_default();

        let offset = 1 + name.len() + 1;

        let oid = i32::from_be_bytes(
            buffer[offset..(offset + 4)].try_into().unwrap(),
        );
        let type_modifier = i32::from_be_bytes(
            buffer[(offset + 4)..(offset + 8)].try_into().unwrap(),
        );

        Self {
            flags,
            name,
            oid,
            type_modifier,
        }
    }
}

pub fn parse_relation_columns(
    column_count: i16,
    buffer: &[u8],
) -> Vec<RelationColumn> {
    let mut columns = Vec::with_capacity(column_count as usize);
    let mut cursor = 0;

    for _ in 0..column_count {
        if buffer.is_empty() {
            break;
        }

        let column = RelationColumn::from_buffer(&buffer[cursor..]);
        cursor += column.get_buffer_size();
        columns.push(column);
    }

    columns
}
