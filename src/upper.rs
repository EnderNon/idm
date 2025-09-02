use leptos::prelude::*;

#[component]
pub fn Upper(
    pkg_name: String,
    pkg_ver: String,
    commit_short: String,
    commit_long: String,
    branch: String,
    buildmode: bool,
) -> impl IntoView {
    view! {
        <div class="upper">
            <div class="upper-left"> // empty, just stretches it out
                <img style="height: 100%; width: 100%; object-fit: scale-down" src="content/img/wyntil.png"/>
            </div>
            <div class="upper-middle">
                "Wynntils Identification Mangler v0.01"
            </div>
            <div class="upper-left"> // empty, just stretches it out
                <a href="https://github.com/Wynntils/Wynntils/issues/2246">"info"</a>
            </div>
        </div>
    }
}
