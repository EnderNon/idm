use crate::{
    helper::{CheckedBox, GenDropdown, IntInput, ShinyViewer, Title},
    json::jsonstruct::{GearJsonItem, Shinystruct},
};
use leptos::prelude::*;

// Lower panel #1 - Item Metadata
// Status: Nearly Ready

#[component]
pub fn Metadata(
    #[prop(into)] selected_item: Memo<Option<(String, GearJsonItem)>>,
    name_select_input: RwSignal<String>,
    #[prop(into)] name_list: Signal<Vec<String>>,
    #[prop(into)] selected_shiny: Signal<Option<(Shinystruct, RwSignal<i64>)>>,
    shiny_select_input: RwSignal<String>,
    #[prop(into)] shiny_list: Signal<Vec<String>>,
    reroll_value_cur: RwSignal<u8>,
    shiny_checkbox: RwSignal<bool>
) -> impl IntoView {
    view! {
        <div class="lower-section">
            <Title name="Item Metadata".to_string() />
            <div class="lower-section-content">
                <h3> "Item Name: "
                {move ||
                    if selected_item.read().is_some() {
                        "✅ "
                    }
                    else if name_select_input.get().is_empty() {
                        "[No Name Detected]"
                    }
                    else {
                        "❌ "
                    }
                }
                {name_select_input}
                </h3>
                <GenDropdown items=name_list out=name_select_input name="namelist".to_string() />
                <br/><br/><br/>

                // <label class="checkbox-contain"> "Enable Shiny: "
                //     <input type="checkbox" bind:checked=shiny_checkbox />
                //     <span class="checkmark" />
                // </label>
                <CheckedBox label={"Enable Shiny"} binded=shiny_checkbox />
                <Show when={move || shiny_checkbox.get()}>
                    <ShinyViewer
                        shiny_select_cur=shiny_select_input
                        selected_shiny=selected_shiny
                    />
                    <GenDropdown
                        items=shiny_list
                        out=shiny_select_input
                        name="shinylist"
                    />
                    <br/>
                    <div class="dual-container">
                    {move || if let Some((_, shiny_value)) = selected_shiny.get() {
                        Some(view! {
                                <div class="text">
                                    "Shiny stat value: "
                                </div>

                                <div class="input">
                                    <IntInput
                                        value=shiny_value
                                        width="12.5em".to_string()
                                    />
                                </div>
                            })
                        } else {
                            None
                        }}
                    </div>
                    <br/>
                </Show>
                <br/>
                <IntInput name="Reroll Value".to_string() range=(0,255) value=reroll_value_cur width="5em".to_string() slider=true />
            </div>
        </div>
    }
}

// Lower panel #2 - Identifications
// Status: UNIMPLEMENTED
