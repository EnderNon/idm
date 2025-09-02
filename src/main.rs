#![allow(unused_variables)]
#![allow(dead_code)]
use leptos::prelude::*;
use lower::Lower;
use shadow_rs::shadow;

mod encode;
mod helper;
mod json;
mod lower;
mod types;
mod upper;

use upper::Upper;

shadow!(shadow);

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}

#[component]
pub fn App() -> impl IntoView {
    let pkg_version = env!("CARGO_PKG_VERSION");
    let pkg_name = env!("CARGO_PKG_NAME");

    // this is stuff for getting the commit ids etc
    let mut shadow_commit_short = shadow::SHORT_COMMIT;
    let mut shadow_commit_long = shadow::COMMIT_HASH;
    let mut shadow_branch = shadow::BRANCH;

    if shadow_commit_short.is_empty() {
        shadow_commit_short = "???"
    }
    if shadow_commit_long.is_empty() {
        shadow_commit_long = "??????"
    };
    if shadow_branch.is_empty() {
        shadow_branch = "main"
    };

    let buildmode_0 = shadow::BUILD_RUST_CHANNEL.trim();
    let buildmode_1 = if buildmode_0 == "release" {
        true
    } else if buildmode_0 == "debug" {
        false
    } else {
        // this HAS to be here sorry
        todo!()
    };

    view! {
        <div class="outer">
            <Upper
                pkg_name={pkg_name.to_string()}
                pkg_ver=pkg_version.to_string()
                commit_short=shadow_commit_short.to_string()
                commit_long=shadow_commit_long.to_string()
                branch=shadow_branch.to_string()
                buildmode=buildmode_1
            >
            </Upper>
            <Suspense fallback = move ||
                view! {<Loading/>} // while waiting for suspense, do this spinny mpreg loading screen
            >
                <Lower/>
            </Suspense>
        </div>
    }
}

#[component]
fn Loading() -> impl IntoView {
    view! {
        <div class="loading">
            <h1> "LOADING!!!!!"</h1>
            <img src="content/img/wyntil.svg"></img>
        </div>
    }
}

/// Error view thing
#[component]
fn ErrView(msg: String) -> impl IntoView {
    view! {
        <h1> "ERROR!!!" </h1>
        <br/>
        <h2> {msg} </h2> // displays the err message provided to the function
        <br/>
        <h2>
            "This was so badly unrecoverable that you should file an issue here: "
            <a href = "https://github.com/EnderNon/idm/issues"> "HERE" </a>
        </h2>

    }
    .into_any()
}
