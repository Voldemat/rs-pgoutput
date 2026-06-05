pub trait StreamingValueTrait {
    const ENABLED: bool;

    fn to_str() -> &'static str;
}

pub struct StreamingValueTraitOn {}
impl StreamingValueTrait for StreamingValueTraitOn {
    const ENABLED: bool = true;

    fn to_str() -> &'static str {
        "on"
    }
}

pub struct StreamingValueTraitParallel {}
impl StreamingValueTrait for StreamingValueTraitParallel {
    const ENABLED: bool = true;

    fn to_str() -> &'static str {
        "parallel"
    }
}

pub struct StreamingValueTraitOff {}
impl StreamingValueTrait for StreamingValueTraitOff {
    const ENABLED: bool = false;

    fn to_str() -> &'static str {
        "off"
    }
}

pub trait BinaryValueTrait: std::fmt::Debug {
    const BINARY: bool;

    fn to_str() -> &'static str;
}

pub struct BinaryValueTraitOn {}
pub struct BinaryValueTraitOff {}

impl BinaryValueTrait for BinaryValueTraitOn {
    const BINARY: bool = true;

    fn to_str() -> &'static str {
        return "true";
    }
}

impl std::fmt::Debug for BinaryValueTraitOn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("BinaryValueTraitOn")
    }
}

impl BinaryValueTrait for BinaryValueTraitOff {
    const BINARY: bool = false;

    fn to_str() -> &'static str {
        return "false";
    }
}

impl std::fmt::Debug for BinaryValueTraitOff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("BinaryValueTraitOff")
    }
}

pub trait MessagesValueTrait {
    fn to_str() -> &'static str;
}
pub struct MessagesValueTraitOn {}
impl MessagesValueTrait for MessagesValueTraitOn {
    fn to_str() -> &'static str {
        "true"
    }
}
pub struct MessagesValueTraitOff {}
impl MessagesValueTrait for MessagesValueTraitOff {
    fn to_str() -> &'static str {
        "false"
    }
}

pub trait TwoPhaseValueTrait {
    fn to_str() -> &'static str;
}
pub struct TwoPhaseValueTraitOn {}
impl TwoPhaseValueTrait for TwoPhaseValueTraitOn {
    fn to_str() -> &'static str {
        "true"
    }
}
pub struct TwoPhaseValueTraitOff {}
impl TwoPhaseValueTrait for TwoPhaseValueTraitOff {
    fn to_str() -> &'static str {
        "false"
    }
}

pub trait OriginValueTrait {
    fn to_str() -> &'static str;
}
pub struct OriginValueTraitAny {}
impl OriginValueTrait for OriginValueTraitAny {
    fn to_str() -> &'static str {
        "any"
    }
}
pub struct OriginValueTraitNone {}
impl OriginValueTrait for OriginValueTraitNone {
    fn to_str() -> &'static str {
        "none"
    }
}

pub fn build_pgoutput_static_options_string_parts<
    Binary: BinaryValueTrait,
    Streaming: StreamingValueTrait,
    Messages: MessagesValueTrait,
    TwoPhase: TwoPhaseValueTrait,
    Origin: OriginValueTrait,
>() -> String {
    format!(
        "proto_version '4', binary '{}', messages: '{}', streaming: '{}', two_phase: '{}', origin: '{}'",
        Binary::to_str(),
        <Messages as MessagesValueTrait>::to_str(),
        Streaming::to_str(),
        <TwoPhase as TwoPhaseValueTrait>::to_str(),
        <Origin as OriginValueTrait>::to_str(),
    )
}
