use super::*;

#[component]
pub fn Input<'a>(name: &'a str) {
    let elem = "another_Rando_ID";
    let ty = "keyup";

    Updater::update(format!("#{elem}"), ty);

    rsx! {
        <input id={elem}
        on_input={name}
        class={"rounded border border-blue-400 bg-blue-100 text-neutral-700 p-2 m-2 placeholder:Enter"}
        placeholder={"Search for anything..."}
        value={name}
        />
    }
}
