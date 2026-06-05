#[repr(u8)]
pub enum BaseEventType {
    BEGIN = b'B',
    COMMIT = b'C',
    RELATION = b'R',
    TYPE = b'Y',
    INSERT = b'I',
    UPDATE = b'U',
    DELETE = b'D',
    TRUNCATE = b'T',
}

impl BaseEventType {
    pub fn from_char(c: u8) -> Option<BaseEventType> {
        if c == BaseEventType::BEGIN as u8 {
            Some(BaseEventType::BEGIN)
        } else if c == BaseEventType::COMMIT as u8 {
            Some(BaseEventType::COMMIT)
        } else if c == BaseEventType::RELATION as u8 {
            Some(BaseEventType::RELATION)
        } else if c == BaseEventType::TYPE as u8 {
            Some(BaseEventType::TYPE)
        } else if c == BaseEventType::INSERT as u8 {
            Some(BaseEventType::INSERT)
        } else if c == BaseEventType::UPDATE as u8 {
            Some(BaseEventType::UPDATE)
        } else if c == BaseEventType::DELETE as u8 {
            Some(BaseEventType::DELETE)
        } else if c == BaseEventType::TRUNCATE as u8 {
            Some(BaseEventType::TRUNCATE)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum BaseEvent<
    Binary: super::tuple_data::PGValue,
    Streaming: crate::options::StreamingValueTrait
        + super::relation::RelationTrait
        + super::truncate::TruncateTrait
        + super::type_::TypeTrait
        + super::insert::InsertTrait<Binary>
        + super::update::UpdateTrait<Binary>
        + super::delete::DeleteTrait<Binary>,
> {
    Begin(super::begin::Begin),
    Commit(super::commit::Commit),
    Relation(<Streaming as super::relation::RelationTrait>::Type),
    Type(<Streaming as super::type_::TypeTrait>::Type),
    Insert(<Streaming as super::insert::InsertTrait<Binary>>::Type),
    Update(<Streaming as super::update::UpdateTrait<Binary>>::Type),
    Delete(<Streaming as super::delete::DeleteTrait<Binary>>::Type),
    Truncate(<Streaming as super::truncate::TruncateTrait>::Type),
}

impl<
    Binary: super::tuple_data::PGValue,
    Streaming: crate::options::StreamingValueTrait
        + super::relation::RelationTrait
        + super::truncate::TruncateTrait
        + super::type_::TypeTrait
        + super::insert::InsertTrait<Binary>
        + super::update::UpdateTrait<Binary>
        + super::delete::DeleteTrait<Binary>,
> BaseEvent<Binary, Streaming>
{
    pub fn parse(
        event_type: &BaseEventType,
        buffer: &[u8],
    ) -> Result<BaseEvent<Binary, Streaming>, String> {
        match event_type {
            BaseEventType::BEGIN => crate::utils::parse_static_size_event::<
                super::begin::Begin,
            >(buffer)
            .map(|value| BaseEvent::Begin(value)),
            BaseEventType::COMMIT => crate::utils::parse_static_size_event::<
                super::commit::Commit,
            >(buffer)
            .map(|value| BaseEvent::Commit(value)),
            BaseEventType::RELATION => {
                crate::utils::parse_dynamic_size_event::<
                    <Streaming as super::relation::RelationTrait>::Type,
                >(buffer)
                .map(|value| BaseEvent::Relation(value))
            }
            BaseEventType::TYPE => crate::utils::parse_dynamic_size_event::<
                <Streaming as super::type_::TypeTrait>::Type,
            >(buffer)
            .map(|value| BaseEvent::Type(value)),
            BaseEventType::INSERT => crate::utils::parse_dynamic_size_event::<
                <Streaming as super::insert::InsertTrait<Binary>>::Type,
            >(buffer)
            .map(|value| BaseEvent::Insert(value)),
            BaseEventType::UPDATE => crate::utils::parse_dynamic_size_event::<
                <Streaming as super::update::UpdateTrait<Binary>>::Type,
            >(buffer)
            .map(|value| BaseEvent::Update(value)),
            BaseEventType::DELETE => crate::utils::parse_dynamic_size_event::<
                <Streaming as super::delete::DeleteTrait<Binary>>::Type,
            >(buffer)
            .map(|value| BaseEvent::Delete(value)),
            BaseEventType::TRUNCATE => {
                crate::utils::parse_dynamic_size_event::<
                    <Streaming as super::truncate::TruncateTrait>::Type,
                >(buffer)
                .map(|value| BaseEvent::Truncate(value))
            }
        }
    }
}
