#[derive(Debug)]
pub enum Event<
    Binary: crate::options::BinaryValueTrait
        + super::base::tuple_data::PGValue
        + std::fmt::Debug,
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
    Binary: crate::options::BinaryValueTrait
        + super::base::tuple_data::PGValue
        + std::fmt::Debug,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let base64_buffers = [
            "UgAAQTlwdWJsaWMAZmlsZXMAZgALAWlkAAAAC4b/////AWNyZWF0ZWRfYXQAAAAEWv////8BdXBkYXRlZF9hdAAAAARa/////wFwYXRoAAAABBP/////AXR5cGUAAAAEE/////8BbmFtZQAAAAQTAAAB+AFzaXplAAAAABf/////AWNoZWNrc3VtAAAABBMAAAAkAXVzZXJfaWQAAAALhv////8BaXNfcHJvY2Vzc2VkAAAAABD/////AXBhcmVudF9pZAAAAAuG/////w==",
            "SQAAQTlOAAt0AAAAJDQ4OWY4NGQ1LWQzMDQtNGE4My04ZWE0LWM4NjQxMmM5Zjk0ZnQAAAAaMjAyNi0wNi0wNiAxMjoyOToxMi43MTQ5NzR0AAAAGjIwMjYtMDYtMDYgMTI6Mjk6MTIuNzE0OTc0dAAAAJRodHRwczovL3MzLnYtdHJlbmR5LnJ1L3F1aWNrY2xpY2svMmNiOTFkMWEtNGNlMy00NzVhLWExY2YtNjc5Y2ZkOTdlNzUzX2U5YTYyZTVlLWVhYmMtNDUxNS1iODk0LTlmZTVkN2YyYjBhYV83ZDVmMDliZS0zZmY0LTQ5ODgtYTFkYy00YzcxODM2NDE3ODQuanBndAAAAAppbWFnZS9qcGVndAAAAHIyY2I5MWQxYS00Y2UzLTQ3NWEtYTFjZi02NzljZmQ5N2U3NTNfZTlhNjJlNWUtZWFiYy00NTE1LWI4OTQtOWZlNWQ3ZjJiMGFhXzdkNWYwOWJlLTNmZjQtNDk4OC1hMWRjLTRjNzE4MzY0MTc4NC5qcGd0AAAABTk0NTI2dAAAACA5NTc5YmExZmI5YWZmZmE3ZTZhYzEyNDk4MzU0MTBkZm50AAAAAWZu",
            "VQAAQTlPAAt0AAAAJGVjMWEzOTk5LWVhODMtNGU0Ny1iNmRjLWYwZGM5OWY3NWFjMnQAAAAaMjAyNi0wMi0yNCAwNzo0ODo0My4zNDE2NDN0AAAAGjIwMjYtMDItMjQgMDc6NDg6NDMuMzQxNjQzdAAAAHFodHRwOi8vczM6OTAwMC9xdWlja2NsaWNrLzU5NGIxZTNmLTcxMWYtNDE4OC05YTk3LTUzYTA3MDM3M2JlMF82OWFjZDdmOS1hYjk0LTRjNmUtYjFlYi1iMzdhMDkwNjdmMDhfSU1HXzMwNTcuanBlZ3QAAAAKaW1hZ2UvanBlZ3QAAABXNTk0YjFlM2YtNzExZi00MTg4LTlhOTctNTNhMDcwMzczYmUwXzY5YWNkN2Y5LWFiOTQtNGM2ZS1iMWViLWIzN2EwOTA2N2YwOF9JTUdfMzA1Ny5qcGVndAAAAAY1MTg2MzF0AAAAIDE2ZGQxOTg3MTY3MWFlYjUzYzc1ZTBjZjg5Y2U3MWVkbnQAAAABZm5OAAt0AAAAJGVjMWEzOTk5LWVhODMtNGU0Ny1iNmRjLWYwZGM5OWY3NWFjMnQAAAAaMjAyNi0wMi0yNCAwNzo0ODo0My4zNDE2NDN0AAAAGjIwMjYtMDItMjQgMDc6NDg6NDMuMzQxNjQzdAAAAHFodHRwOi8vczM6OTAwMC9xdWlja2NsaWNrLzU5NGIxZTNmLTcxMWYtNDE4OC05YTk3LTUzYTA3MDM3M2JlMF82OWFjZDdmOS1hYjk0LTRjNmUtYjFlYi1iMzdhMDkwNjdmMDhfSU1HXzMwNTcuanBlZ3QAAAAKaW1hZ2UvanBlZ3QAAABXNTk0YjFlM2YtNzExZi00MTg4LTlhOTctNTNhMDcwMzczYmUwXzY5YWNkN2Y5LWFiOTQtNGM2ZS1iMWViLWIzN2EwOTA2N2YwOF9JTUdfMzA1Ny5qcGVndAAAAAY1MTg2MzF0AAAAIDE2ZGQxOTg3MTY3MWFlYjUzYzc1ZTBjZjg5Y2U3MWVkbnQAAAABZm4="
        ];
        for base64_buffer in base64_buffers {
            let buffer = base64::Engine::decode(
                &base64::engine::general_purpose::STANDARD,
                base64_buffer,
            )
            .unwrap();

            let event_type = EventType::from_char(buffer[0]).unwrap();

            println!("{:?}", Event::<
                crate::options::BinaryValueTraitOff,
                crate::options::StreamingValueTraitOff,
            >::parse(&event_type, &buffer[1..])
            .unwrap());
        }
    }
}
