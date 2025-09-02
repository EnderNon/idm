use crate::{
    encode::encode_output_string,
    helper::{HGen, Title},
    json::jsonstruct::{GearJsonItem, Shinystruct},
    types::IdentificationSignal,
};
use idmangler_lib::{block::IdentificationData, types::EncodingVersion};
use leptos::control_flow::Show;
use leptos::prelude::*;

#[component]
pub fn Preview(
    selected_item: Memo<Option<(String, GearJsonItem)>>,
    reroll_value_cur: RwSignal<u8>,
    #[prop(into)] selected_shiny: Signal<Option<(Shinystruct, RwSignal<i64>)>>,
    #[prop(into)] shiny_preview: Signal<bool>,
    extended_ids: RwSignal<bool>,
    identifications: RwSignal<Vec<IdentificationSignal>>,
) -> impl IntoView {
    let item_name_display = Memo::new(move |_| match selected_item.get() {
        Some((name, _)) => name,
        None => String::from("[Unknown Name]"),
    });

    // output calcs
    let fr = Memo::new(move |_| {
        let a = encode_output_string(
            selected_item.get(),
            selected_shiny.get().map(|s| (s.0, s.1.get())),
            shiny_preview.get(),
            reroll_value_cur.get(),
            EncodingVersion::Version1,
            IdentificationData {
                identifications: identifications.get().iter().map(|&i| i.into()).collect(),
                extended_encoding: extended_ids.get(),
            },
        );

        a.unwrap_or_default()
    });

    view! {
        <div class="lower-section">
            <Title name="Preview".to_string()/>
            <div class="lower-section-content">
                <HGen
                    name=item_name_display
                    r#prefix={
                        Signal::derive(move || 
                            if shiny_preview.get() {
                                selected_shiny.read().is_some().then_some("Shiny".to_string())
                            } else {Some("".to_string())}
                        )
                    }
                />
                <Show when=move || shiny_preview.get() >
                    {move || selected_shiny.get().map(|(shiny, value)| {
                        view! {
                            {shiny.disname} ": " {value}
                            <br/>
                        }
                    })}
                </Show>

                <Show when={move || reroll_value_cur.get()>0}>
                    <h3>
                        "[" {reroll_value_cur} "]"
                    </h3>
                </Show>

            </div>
            <Show when={move || !fr.get().is_empty()}>
                <div class="lower-section-content">
                    <h2> "Item: " </h2>
                    {move || fr.get()}
                </div>
            </Show>
        </div>
    }
    .into_any()
}
