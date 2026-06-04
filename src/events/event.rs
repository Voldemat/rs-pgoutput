pub enum Event<
    Binary: crate::options::BinaryValueTrait + super::base::tuple_data::PGValue,
    Streaming: crate::options::StreamingValueTrait
        + super::message::MessageTrait
        + super::base::relation::RelationTrait
        + super::base::type_::TypeTrait
        + super::base::insert::InsertTrait<Binary>
        + super::base::update::UpdateTrait<Binary>
        + super::base::delete::DeleteTrait<Binary>
        + super::base::truncate::TruncateTrait
        + super::stream::StreamAbortTrait,
> {
    Base(super::base::event::BaseEvent<Binary, Streaming>),
    // if Messages == MessagesValue::ON
    Message(<Streaming as super::message::MessageTrait>::Type),
    // if OriginConf == OriginValue::ANY
    Origin(super::origin::Origin),
    // if StreamingEnabled == StreamingEnabledValue::ON
    Stream(super::stream::StreamEvent<Streaming>),
    // if TwoPhase == TwoPhaseValue::ON
    TwoPhase(super::twophase::TwoPhaseCommitEvent),
    // if TwoPhase == TwoPhaseValue::ON and Streaming == StreamingValue::ON
    StreamPrepare(super::stream_and_two_phase::StreamPrepare),
}

impl<
    Binary: crate::options::BinaryValueTrait + super::base::tuple_data::PGValue,
    Streaming: crate::options::StreamingValueTrait
        + super::message::MessageTrait
        + super::base::relation::RelationTrait
        + super::base::type_::TypeTrait
        + super::base::insert::InsertTrait<Binary>
        + super::base::update::UpdateTrait<Binary>
        + super::base::delete::DeleteTrait<Binary>
        + super::base::truncate::TruncateTrait
        + super::stream::StreamAbortTrait,
> Event<Binary, Streaming>
{
    pub fn parse(
        event_type: &EventType,
        buffer: &[u8],
    ) -> Result<Event<Binary, Streaming>, String> {
        match event_type {
            EventType::Base(base_event_type) => {
                super::base::event::BaseEvent::parse(base_event_type, buffer)
                    .map(|event| Event::Base(event))
            }
            EventType::Message => crate::utils::parse_dynamic_size_event::<
                <Streaming as super::message::MessageTrait>::Type,
            >(buffer)
            .map(|event| Event::Message(event)),
            EventType::Origin => crate::utils::parse_dynamic_size_event::<
                super::origin::Origin,
            >(buffer)
            .map(|event| Event::Origin(event)),
            EventType::Stream(stream_event_type) => {
                super::stream::parse_streaming_event(stream_event_type, buffer)
                    .map(|event| Event::Stream(event))
            }
            EventType::TwoPhase(twophase_event_type) => {
                super::twophase::parse_two_phase_commit_event(
                    twophase_event_type,
                    buffer,
                )
                .map(|event| Event::TwoPhase(event))
            }
            EventType::StreamPrepare => {
                crate::utils::parse_dynamic_size_event::<
                    super::stream_and_two_phase::StreamPrepare,
                >(buffer)
                .map(|event| Event::StreamPrepare(event))
            }
        }
    }
}

pub enum EventType {
    Base(super::base::event::BaseEventType),
    // if Messages == MessagesValue::ON
    Message,
    // if OriginConf == OriginValue::ANY
    Origin,
    // if StreamingEnabled == StreamingEnabledValue::ON
    Stream(super::stream::StreamEventType),
    // if TwoPhase == TwoPhaseValue::ON
    TwoPhase(super::twophase::TwoPhaseCommitEventType),
    // if TwoPhase == TwoPhaseValue::ON and Streaming == StreamingValue::ON
    StreamPrepare,
}

impl EventType {
    pub fn from_char(c: u8) -> Option<Self> {
        super::base::event::BaseEventType::from_char(c)
            .map(|e| EventType::Base(e))
            .or_else(|| {
                if c == super::message::TYPE_DESCRIMINATOR {
                    Some(EventType::Message)
                } else {
                    None
                }
            })
            .or_else(|| {
                if c == super::origin::TYPE_DESCRIMINATOR {
                    Some(EventType::Origin)
                } else {
                    None
                }
            })
            .or_else(|| {
                super::stream::StreamEventType::from_char(c)
                    .map(|e| EventType::Stream(e))
            })
            .or_else(|| {
                super::twophase::TwoPhaseCommitEventType::from_char(c)
                    .map(|e| EventType::TwoPhase(e))
            })
            .or_else(|| {
                if c == super::stream_and_two_phase::TYPE_DESCRIMINATOR {
                    Some(EventType::StreamPrepare)
                } else {
                    None
                }
            })
    }
}
