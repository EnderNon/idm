use leptos::prelude::RwSignal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::{IdentificationSignal, RollTypeSignal};

/// The struct for gear.json.
/// Stored as json Hashmap<String, GearJson>.
///
/// See https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/gear.json
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct GearJsonItem {
    pub identifications: Option<HashMap<String, GearJsonIdValue>>,
}

impl GearJsonItem {
    /// Converts identifications from the gear.json to a Vec of idmangler-lib Stat.
    pub fn into_id_signals(self, id_keys: &HashMap<String, u8>) -> Vec<IdentificationSignal> {
        let mut out = Vec::new();

        if let Some(idents) = &self.identifications {
            for (k, v) in idents {
                // TODO: error handling?
                let id_kind = id_keys.get(k).copied().unwrap_or_default();

                match v {
                    GearJsonIdValue::Range(val) => out.push(IdentificationSignal {
                        kind: RwSignal::new(id_kind),
                        base: RwSignal::new(val.raw),
                        roll: RwSignal::new(RollTypeSignal::Value(RwSignal::new(130))),
                    }),
                    GearJsonIdValue::PreId(val) => out.push(IdentificationSignal {
                        kind: RwSignal::new(id_kind),
                        base: RwSignal::new(*val),
                        roll: RwSignal::new(RollTypeSignal::PreIdentified),
                    }),
                }
            }
        }

        out
    }
}

/// Enum representing an identification out of the gear.json.
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
#[serde(untagged)]
pub enum GearJsonIdValue {
    // A range of values with a base value
    Range(GearJsonIdentificationRange),
    // A fixed predefined value
    PreId(i32),
}

/// A range of possible values for a given identification.
///
/// The `raw` field is the base value used in encoding.
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct GearJsonIdentificationRange {
    max: i32,
    min: i32,
    /// Identification base value
    raw: i32,
}
/// this [`DlJsons`] struct is basically just containing all three of relevant wynntils jsons, as three elements of a struct
#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct DlJsons {
    /// Literally just a list of Identifications and their internal enum numbers, or something like that.  
    /// See <https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/id_keys.json>
    pub id_keys: HashMap<String, u8>,

    /// A list of potential Shiny Mythic types.  
    /// See <https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Data-Storage/shiny_stats.json>
    pub shiny_stats: Vec<Shinystruct>,
    pub gear: HashMap<String, GearJsonItem>,
}

/// Data from `shiny_stats.json`.
/// Stored as json Vec\<Shinystruct\>.
///
/// See <https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Data-Storage/shiny_stats.json>
#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Shinystruct {
    /// an internal enum "id" as integer.
    pub id: u8,
    /// a "key" in lowerCamelCase.
    pub key: String,
    /// a "displayName" as formatted text. e.g. "Chests Opened"
    #[serde(rename = "displayName")]
    pub disname: String,
    /// i'm not even sure what this is,
    /// but all of them have value "raw".
    /// perhaps this is for future compatibility?
    #[serde(rename = "statUnit")]
    pub statunit: String,
}
