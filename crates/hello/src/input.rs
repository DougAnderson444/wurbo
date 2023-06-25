use super::*;

#[component]
pub fn Input<'a>(title: &'a str) {
    rsx! {
        <input
        on_input={title}
        class={"rounded border border-blue-400 bg-blue-100 text-neutral-700 p-2 m-2 placeholder:Enter"}
        placeholder={"Search for anything..."}
        value={title}
        />
    }
}
