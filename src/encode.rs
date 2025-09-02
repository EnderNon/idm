use crate::json::jsonstruct::{GearJsonItem, Shinystruct};
use idmangler_lib::{
    block::{EndData, IdentificationData, NameData, RerollData, ShinyData, StartData, TypeData},
    encoding::{DataEncoder, string::encode_string},
    types::{EncodingVersion, ItemType},
};

pub fn encode_output_string(
    selected_item: Option<(String, GearJsonItem)>,
    // current shiny data.
    // shiny type, followed by value.
    selected_shiny: Option<(Shinystruct, i64)>,
    // Enable shiny or not
    shiny_enable: bool,
    reroll: u8,
    ver: EncodingVersion,
    ident_data: IdentificationData,
) -> Option<String> {
    let selected_item = selected_item?;
    let mut out: Vec<u8> = Vec::new();

    // ENCODE: StartData (required)
    StartData(ver).encode(ver, &mut out).unwrap();

    // ENCODE: TypeData (required)
    // for now only gear is implemented while i'm still figuring out how to make this thing work
    TypeData(ItemType::Gear).encode(ver, &mut out).unwrap();

    // ENCODE: NameData (required)
    NameData(selected_item.0.clone())
        .encode(ver, &mut out)
        .unwrap();

    // ENCODE: IdentificationData (optional)
    ident_data.encode(ver, &mut out).unwrap();

    // ENCODE: PowderData (optional)
    // TODO

    // ENCODE: RerollData (optional)
    if reroll != 0 {
        RerollData(reroll).encode(ver, &mut out).unwrap()
    }

    // ENCODE: ShinyData (optional)
    if shiny_enable {
        if let Some(shiny) = selected_shiny {
            ShinyData {
                id: shiny.0.id,
                val: shiny.1,
            }
            .encode(ver, &mut out)
            .unwrap()
        }
    }

    // ENCODE: EndData (required)
    EndData.encode(ver, &mut out).unwrap();

    Some(encode_string(&out))
}
