use std::any::type_name;

pub trait StaticSizeEvent: Sized {
    const BUFFER_SIZE: usize;
    fn from_buffer(buffer: &[u8]) -> Self;
}

pub fn parse_static_size_event<
    T: StaticSizeEvent,
>(
    buffer: &[u8],
) -> Result<T, String> {
    if buffer.len() != T::BUFFER_SIZE {
        return Err(format!(
            "{} event buffer size must be {}",
            type_name::<T>(),
            T::BUFFER_SIZE
        ));
    }

    Ok(T::from_buffer(buffer))
}

pub trait DynamicSizeEvent: Sized {
    const MIN_BUFFER_SIZE: usize;
    fn from_buffer(buffer: &[u8]) -> Self;
}

pub fn parse_dynamic_size_event<T: DynamicSizeEvent>(
    buffer: &[u8],
) -> Result<T, String> {
    if buffer.len() < T::MIN_BUFFER_SIZE {
        return Err(format!(
            "{} event buffer size must be gte {}",
            type_name::<T>(),
            T::MIN_BUFFER_SIZE
        ));
    }

    Ok(T::from_buffer(buffer))
}
