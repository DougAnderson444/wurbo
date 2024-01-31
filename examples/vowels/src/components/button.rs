use super::*;

#[component]
pub fn Button<'a, Children: Render>(title: &'a str, children: Children) {
    rsx! {
      <>
        <button on_click={title} class={"rounded bg-green-500 text-white p-2 m-2 shadow"}>
            {children}
        </button>
      </>
    }
}
