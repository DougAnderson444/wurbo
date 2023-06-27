use super::*;
use components::count::count_vowels;

#[component]
pub fn Output<'a>(name: &'a str) {
    let count = count_vowels(name);
    let id = "random_output_id";

    rsx! {
        <div id>
            <br/>
            {format!("Hello, {name}!")}<br/>
            {format!("{name} has {count} vowels.")}<br/>
        </div>
    }
}
