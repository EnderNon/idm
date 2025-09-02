// panel 1 - metadata
pub mod metadata;
// panel 2 - identifications
pub mod identification;
// panel 3 - powders
pub mod powder;
// powder 4 - preview (+ item render)
pub mod preview;

// lower panel stuff below

use identification::Identifications;
use leptos::{prelude::*, server::LocalResource};
use metadata::Metadata;
use powder::Powders;
use preview::Preview;

use crate::{
    helper::get_key_list_sorted,
    json::{jsondl::fetch_all, jsonstruct::*},
    types::IdentificationSignal,
};
use std::collections::HashMap;

#[component]
pub fn Lower() -> impl IntoView {
    let lr = LocalResource::new(async move || {
        fetch_all(
        "https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/id_keys.json",
        "https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Data-Storage/shiny_stats.json",
        "https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/gear.json"
    ).await.map_err(|e| e.to_string())
    }); // map a potential Err given to String because otherwise it cannot be Cloned
    // .get() on LocalResource<T> gives you Option<SendWrapper<T>>
    // this change below transforms to just Option<T>
    // thanks zatzou
    let fr_data = Memo::new(move |_| lr.get().map(|d| d.take()));

    let fr_data2 = move || match fr_data.get().ok_or(String::new()) {
        Ok(inner) => inner,
        Err(e) => Err(e),
    };

    let wynntils_data = move || match fr_data2() {
        Ok(somedata) => somedata,
        Err(_) => DlJsons {
            id_keys: HashMap::new(),
            shiny_stats: Vec::new(),
            gear: HashMap::new(),
        },
    };

    // variable inits here
    // Yes, the type definitions are intentional, because otherwise it becomes really confusing to track stuff

    // checkbox whether shiny is enabled or not
    let shiny_checkbox: RwSignal<bool> = RwSignal::new(false);

    // Current entered item name in the first panel
    let name_select_input: RwSignal<String> = RwSignal::new(String::new());
    // List of Wynntils gear, used to search in the item name dropdown in first panel
    let gear_namelist: Memo<Vec<String>> =
        Memo::new(move |_| get_key_list_sorted(&wynntils_data().gear));

    // Memo for the currently selected item
    // This memo is Some(name, item_value) if the selected name is valid, otherwise None
    let selected_item = Memo::new(move |_| {
        if let Some((k, v)) = wynntils_data()
            .gear
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name_select_input.read().as_str()))
        {
            Some((k.clone(), v.clone()))
        } else {
            None
        }
    });

    // Currently selected shiny
    let shiny_select_input: RwSignal<String> = RwSignal::new(String::new());

    // Get the shiny stats from the wynntils data
    let shiny_vec: Memo<Vec<Shinystruct>> = Memo::new(move |_| wynntils_data().shiny_stats);
    let shiny_list: Memo<Vec<String>> = Memo::new(move |_| {
        let mut a: Vec<String> = shiny_vec
            .get()
            .iter()
            .map(|fr| fr.disname.clone())
            .collect::<Vec<String>>();
        a.sort();
        a
    });

    let selected_shiny: Memo<Option<(Shinystruct, RwSignal<i64>)>> = Memo::new(move |_| {
        shiny_vec
            .read()
            .iter()
            .find(|sh| {
                sh.disname
                    .eq_ignore_ascii_case(shiny_select_input.read().as_str())
            })
            .map(|sh| (sh.clone(), RwSignal::new(0i64)))
    });

    // whether the shiny mode checkbox is ticked or not
    let shiny_checkbox = RwSignal::new(false);

    // Derived from the above two variables. Determines whether to actually do the shiny in the preview.
    let shiny_preview = Memo::new(move |_| shiny_checkbox.get() && selected_shiny.read().is_some());

    let cur_reroll: RwSignal<u8> = RwSignal::new(0u8);

    // whether or not we are using extended encoding for identifications
    let extended_ids: RwSignal<bool> = RwSignal::new(false);
    // the identifications which we are currently dealing with
    let identifications: RwSignal<Vec<IdentificationSignal>> = RwSignal::new(vec![]);

    // I can't even remember anymore tbh
    let id_keys: Memo<HashMap<String, u8>> = Memo::new(move |_| wynntils_data().id_keys);

    // update the identifications when the item changes
    Effect::new(move || {
        if let Some((_, item)) = selected_item.get() {
            identifications.set(item.into_id_signals(&wynntils_data().id_keys));
        } else {
            identifications.set(vec![]);
        }
    });

    // disable animation whenever I need to
    let animate = RwSignal::new(true);

    view! {
        <div class="lower"
            class:fade-in-up={animate}
            on:animationend={move |_| animate.set(false)}
        >
            <Metadata
                selected_item=selected_item
                name_select_input = name_select_input
                name_list = gear_namelist
                selected_shiny = selected_shiny
                shiny_select_input = shiny_select_input
                shiny_list = shiny_list
                shiny_checkbox = shiny_checkbox
                reroll_value_cur = cur_reroll
            />
            <Identifications
                extended = extended_ids
                ids = identifications
                id_keys = id_keys
            />
            <Preview
                selected_item=selected_item
                reroll_value_cur=cur_reroll
                selected_shiny=selected_shiny
                shiny_preview=shiny_preview
                extended_ids=extended_ids
                identifications=identifications
            />
        </div>
    }
}
