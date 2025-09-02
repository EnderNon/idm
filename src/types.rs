use idmangler_lib::types::RollType;
use leptos::prelude::*;

/// An identification with its values as signals.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentificationSignal {
    pub kind: RwSignal<u8>,
    pub base: RwSignal<i32>,
    pub roll: RwSignal<RollTypeSignal>,
}

impl From<IdentificationSignal> for idmangler_lib::types::Stat {
    fn from(value: IdentificationSignal) -> Self {
        Self {
            kind: value.kind.get(),
            base: Some(value.base.get()),
            roll: value.roll.get().into(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum RollTypeSignal {
    Value(RwSignal<u8>),
    PreIdentified,
}

impl From<RollTypeSignal> for RollType {
    fn from(value: RollTypeSignal) -> Self {
        match value {
            RollTypeSignal::Value(signal) => RollType::Value(signal.get()),
            RollTypeSignal::PreIdentified => RollType::PreIdentified,
        }
    }
}
