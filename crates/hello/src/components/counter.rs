use super::*;

#[component]
pub fn Counter<'a, Children: Render>(title: &'a str, children: Children) {
    // on 'click' event, increment the count
    // let elem = "counter";
    // let ty = "click";

    // // add '#' to the elem_id
    // Updater::update(format!("#{elem}"), ty);

    rsx! {
        <div class={"rounded bg-green-500 text-white p-2 m-2 shadow"}>
            {children}
        </div>
    }
}
