use crate::{helper::*, types::*};
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Identifications(
    /// Is extended encoding enabled?
    extended: RwSignal<bool>,
    /// The current identifications
    ids: RwSignal<Vec<IdentificationSignal>>,
    /// id keys from the jsons
    #[prop(into)]
    id_keys: Signal<HashMap<String, u8>>,
) -> impl IntoView {
    let remove_id = move |idx| {
        ids.update(|ids| {
            ids.remove(idx);
        });
    };

    let add_id = move |_| {
        ids.update(|ids| {
            ids.push(IdentificationSignal {
                kind: RwSignal::new(0),
                base: RwSignal::new(0),
                roll: RwSignal::new(RollTypeSignal::PreIdentified),
            });
        });
    };

    view! {
        <div class="lower-section">
            <Title name="Identifications".to_string()/>
            <div class="lower-section-content" style:flex-grow="0">
                <CheckedBox label={"Use extended encoding"} binded=extended />

                <Show when={move || extended.get()}>
                    <Buttoned binded=add_id label="Add identification" />
                </Show>
            </div>
            <div class="lower-section-content" style:flex-grow="1" style:padding="0">
                <ForEnumerate
                    each=move || ids.get()
                    key=move |id| (*id, extended.get())
                    children=move |idx, id| {
                        // only render ids when extended is enabled or the is not pre-identified
                        (extended.get() || matches!(id.roll.get(), RollTypeSignal::Value(_))).then_some(view! {
                            <div style:padding="var(--pad)">
                                <div style:display="flex" style:justify-content="space-between">
                                    {move || if extended.get() {
                                        view! {
                                            <select on:input:target=move |ev| {
                                                let value = ev.target().value();
                                                id.kind.set(value.parse::<u8>().unwrap_or_default());   
                                            }>
                                                <For
                                                    each=move || id_keys.get()
                                                    key=move |i| i.clone()
                                                    children=move |(k, i)| {
                                                        view! {
                                                            <option value=i selected={id.kind.get() == i}>{k}</option>
                                                        }
                                                    }
                                                />
                                            </select>
                                            <Buttoned binded={move |_| remove_id(idx.get())} label="X" />
                                        }.into_any()
                                    } else {
                                        view! {
                                            <h2>{move || id_keys.read().iter().find(|(_, i)| **i == id.kind.get()).map(|(k, _)| k).cloned()}</h2>
                                        }.into_any()
                                    }}
                                </div>

                                // base value selector
                                <Show when={move || extended.get()}>
                                    <IntInput name="Base value" value=id.base width="10em" />
                                </Show>

                                // roll type selector
                                <Show when={move || extended.get()}>
                                    <label>
                                        "Roll type "
                                        <select on:input:target=move |ev| {
                                            let value = ev.target().value();
                                            id.roll.set(if value == "0" {
                                                RollTypeSignal::PreIdentified
                                            } else {
                                                RollTypeSignal::Value(RwSignal::new(130))
                                            });
                                        }>
                                            <option value="0" selected={move || matches!(id.roll.get(), RollTypeSignal::PreIdentified)}>"Pre-Identified"</option>
                                            <option value="1" selected={move || matches!(id.roll.get(), RollTypeSignal::Value(_))}>"Rolled"</option>
                                        </select>
                                    </label>
                                </Show>

                                // roll selector
                                {move || if let RollTypeSignal::Value(roll) = id.roll.get() {
                                    Some(view! {
                                        <IntInput name="Roll" range=(0, 255) value=roll width="10em" slider=true />
                                    })
                                } else {
                                    None
                                }}
                            </div>

                            <hr style:border="0.1em solid var(--border)"/>
                        })
                    }
                />
            </div>
        </div>
    }
    .into_any()
}
