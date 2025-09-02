use std::collections::HashMap;
use std::str::FromStr;

use leptos::attr::AttributeValue;
use leptos::ev::MouseEvent;
use leptos::prelude::*;

use crate::json::jsonstruct::{DlJsons, Shinystruct};
use crate::types::{IdentificationSignal, RollTypeSignal};

/// Title for each section in the Lower.
/// Basically just a h1 in it's own little div with a seperator between title and content of the section.
#[component]
pub fn Title(name: String) -> impl IntoView {
    view! {
        <div class="lower-section-title">
            <h1>{name}</h1>
        </div>
    }
}

/// H Gen component used in item preview
#[component]
pub fn HGen(#[prop(into)] name: Signal<String>, prefix: Signal<Option<String>>) -> impl IntoView {
    view! {
        <h2>
            // only show prefix if it exists (crazy unwrap)
            {move || prefix.get().map(|pre| view! {{pre} " "})}
            // the name value
            {move || name.get()}
        </h2>
    }
}

/// Element list - currently unused.
/// Fuck off xeondev, I'm not going to add an underscore before the element.
pub enum Element {
    Earth,
    Thunder,
    Water,
    Fire,
    Air,
}

#[component]
pub fn IntInput<N>(
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional)] range: Option<(N, N)>,
    value: RwSignal<N>,
    #[prop(into)] width: String,
    #[prop(optional)] slider: bool,
) -> impl IntoView
where
    N: Clone + Sync + PartialEq + AttributeValue + FromStr + ToString + 'static,
{
    let view = move || value.read().to_string();

    // update the value whenever the event fires
    let tryset = move |val: String| {
        if let Ok(val) = val.parse::<N>() {
            value.set(val);
        }
    };

    view! {
        <form class="pure-form" style:display="block">
            <div style:display="flex" style:justify-content="space-between">
                // name of the input
                {name.map(|fr_name|
                    view!{ <label>{fr_name}</label> }
                )}

                // number input box
                <div class="control">
                    <input
                        style:float="right"
                        style:width={width}

                        prop:type="number"

                        prop:value={move || view()}
                        on:input:target={move |ev| {
                            tryset(ev.target().value());
                        }}
                    />
                </div>
            </div>

            // the slider
            {if slider {Some(view! {
                <div class="control">
                    <input
                        style:width="100%"
                        class="sliding"

                        prop:type="range"
                        // given that "range" variable is an Option tuple of 2 Strings (Min, Max)
                        // If it is Some, the map function will add it into the min and max.
                        // If it is None, then it just spits out an Option::None::<i32>
                        // This is allowed for some reason
                        min={range.clone().map(|r| r.0)}
                        max={range.map(|r| r.1)}

                        prop:value={move || view()}
                        on:input:target={move |ev| {
                            tryset(ev.target().value());
                        }}
                    />
                </div>
            })} else {None} }
        </form>
    }
}

#[component]
pub fn ShinyViewer(
    shiny_select_cur: RwSignal<String>,
    selected_shiny: Signal<Option<(Shinystruct, RwSignal<i64>)>>,
) -> impl IntoView {
    view! {
        <h3>
            "Shiny Stat: " {move ||
                if selected_shiny.read().is_some() {
                    "✅ "
                }
                else if shiny_select_cur.get().is_empty() {
                    "[No Text Detected]"
                }
                else {
                    "❌ "
                }
            }
            {shiny_select_cur}
        </h3>
    }
}

/// Function to check whether some inputted string is in a list of stuff.
/// Uses binary search, so you need a sorted list.
pub fn check_selected(
    whatever_list: Signal<Vec<String>>,
    whatever_string_current: RwSignal<String>,
) -> Memo<bool> {
    Memo::new(move |_| {
        let check_existing = whatever_list
            .get()
            .binary_search(&whatever_string_current.get().trim().to_string());

        check_existing.is_ok()
    })
}

/// For a HashMap\<A.B\>, gets all values of A. Then puts those values into a Vec<A>, sorted alphabetically.
pub fn get_key_list_sorted<A: std::cmp::Ord + std::clone::Clone, B: std::clone::Clone>(
    hashmapped: &HashMap<A, B>,
) -> Vec<A> {
    let mut a: Vec<A> = <HashMap<A, B> as Clone>::clone(hashmapped)
        .into_keys()
        .collect();
    a.sort();
    a
}

/// Generates a dropdown
#[component]
pub fn GenDropdown(
    #[prop(into)] items: Signal<Vec<String>>,
    out: RwSignal<String>,
    #[prop(into)] name: String,
) -> impl IntoView {
    let fr = move || {
        items
            .get()
            .into_iter()
            .map(|frfrnocap| {
                view! {
                    <option value={frfrnocap} />
                }
                .into_any()
            })
            .collect::<Vec<_>>()
    };
    let stringle = out;

    view! {
        <input bind:value=stringle r#type="text" list=name.clone() autocomplete="off" />
        <datalist id={name}>
            {fr}
        </datalist>
    }
}

/// custom styled checkbox
#[component]
pub fn CheckedBox (
    #[prop(into)] label: String,
    binded: RwSignal<bool>
 ) -> impl IntoView {
    view! {
        <label class="checkbox-contain"> {label}
            <input type="checkbox" bind:checked=binded />
            <span class="checkmark" />
        </label>
    }.into_any()
}

/// custom styled button
#[component]
pub fn Buttoned(
    #[prop(into)] label: String,
    binded: impl Fn(MouseEvent) + 'static
) -> impl IntoView {
    view! {
        <button class="buttons" on:click=binded>
            {label}
        </button>
    }
}

#[component]
/// Generates a View for the Preview panel, for showcasing IDs
pub fn IdPreview(
    ids: IdentificationSignal,
) -> impl IntoView {
    let frfrnocap = move || {
        let kind = ids.kind.get();
        let base = ids.base.get();
        let roll = ids.roll.get();
    };
    view! {
        
    }.into_any()
}

#[derive(Clone)]
pub struct IdCalcOut {
    percent: f64,
    name: String,
    outvalue: String
}
/// do calcs on wynntils
pub fn id_calc(
    wynntils: DlJsons,
    kind: u8,
    base: i32,
    roll: RollTypeSignal,
) -> IdCalcOut {

    // I put option just in case. But if this thing does not find a value, I'll be really damn surprised, 
    // since the user shouldn't be able to influence this at all.
    let mapped_id_name = hm_value_lookup(&wynntils.id_keys,&kind)
        .map_or("UnknownID", |v| v);
    
    let positive_or_negative = base >= 0;


    // placeholder value
    IdCalcOut {
        percent: 100.0,
        name: "Mana Regen".into(),
        outvalue: "16/5s".into()
    }
}

/// Function to find the key in a HashMap, given the value. 
/// The only reason why I'm even doing this is because I 
/// represented the Wynntils identification id enum in a 
/// stupid way as a HashMap. Terrible idea but too late to change.
pub fn hm_value_lookup<'a, // Borrowing is intentional, I don't want to be met with Copy issues.
    A,
    B: PartialEq // needed for the comparison
>(
    hasmap: &'a HashMap<A,B>,
    query: &'a B
) -> Option<&'a A> {    
    let test = hasmap.iter().find(|(key,value)| *value == query);
    if let Some((key,value)) = test {
        return Some(key)
    }
    else {
        return None
    }
}