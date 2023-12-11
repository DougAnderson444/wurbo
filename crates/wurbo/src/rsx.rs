/// This macro takes $page, $input, $output and $imports and generates the Trait and the
/// default implementations using the given variables. The macro should also take care of importing
/// the necessary dependencies.
///
/// This macro generates the Trait and the default implementations using the given variables.
/// The macro should also take care of importing the necessary dependencies.
///
/// # Example
/// (disable doctest for this example)
/// ```ignore
/// use wurbo::generate_reactivity;
/// use render::{
///     // A macro to create components
///     component,
///     // A macro to render components in JSX fashion
///     html,
///     // A macro to compose components in JSX fashion
///     rsx,
///     // A trait for custom components
///     Render,
/// };
///
/// use crate::bindings::exports::demo::vowels::reactivity::Guest as WurboGuest;
/// use crate::bindings::demo::vowels::imports; // so the macro has access to the imports::types
///
/// /// The WIT Component struct for implementing the Guest trait
/// struct Component;
///
//? #[component]
/// pub fn Page<'a, Children: Render>(title: &'a str, children: Children) {
///     rsx! {
///       <div class={"p-4"}>
///         <div>
///             {children}
///         </div>
///       </div>
///     }
/// }
///
///
/// #[component]
/// pub fn Input<'a>(name: &'a str, id: &'a str) {
///     // Type of event listener to listen for
///     let ty = "keyup";
///
///     // Add this CSS selector to the list of selectors that will add event listeners
///     super::wurbo_tracker::track(format!("#{id}"), ty);
///
///     rsx! {
///         <div>
///             <div>
///                 {"The data you enter can't be seen by anyone else, since it's in a WebAssembly sandbox. =)"}
///             </div>
///             <input id
///             value={name}
///             />
///             <div>
///                {"But it can still calculate how many vowels are in your words for you!"}
///             </div>
///         </div>
///     }
/// }
///
/// #[component]
/// pub fn Output<'a>(name: &'a str, id: &'a str) {
///     let count = count_vowels(name);
///
///     rsx! {
///         <div id>
///         <b>{name}</b>
///             {
///                 match count {
///                     0 => { " has no vowels.".to_string() } ,
///                     1 => {format!(" has {count} vowel.")}
///                     _ => {format!(" has {count} vowels.")}
///                 }
///             }
///             <br/>
///         </div>
///     }
/// }
///
/// /// The macro combines your components together and injects the reactivity:
/// generate_reactivity! { WurboGuest, Component, Page, Input, Output, imports }
/// ```
#[macro_export]
macro_rules! generate_reactivity {
    ($guest: ident, $component: ident, $page:ident, $input:ident, $output:ident, $imports:ident) => {
        use $crate::prelude::*;

        use std::collections::HashMap;
        use std::sync::Mutex;
        use std::sync::OnceLock;
        use std::sync::RwLock;

        ///Maps the #elementId to the event type
        type ListenerMap = HashMap<String, &'static str>;

        // We cannot have &self in the Component struct,
        // so we use static variables to store the state between functions
        // See https://crates.io/crates/lazy_static
        lazy_static! {
          // create Vec<bindings::component::cargo_comp::imports::ListenDetails>
          static ref LISTENERS_MAP: Mutex<ListenerMap> = Mutex::new(HashMap::new());
          // is_initialized
          static ref IS_INITIALIZED: RwLock<bool> = RwLock::new(false);
        }

        /// The HTML element id of the output section so we can surgically render re-render it
        static OUTPUT_ID: OnceLock<String> = OnceLock::new();

        // unique namespace to clairfy and avoid collisions with other Guest code
        mod wurbo_tracker {
            /// Insert the element id and event type into the LISTENERS_MAP
            ///
            /// # Example
            ///
            /// ```rust
            /// let my_CSS_selector = "#some_selector";
            /// Interactive::activate(format!("#{my_CSS_selector}"), "keyup");
            /// ```
            pub fn track(elem_id: String, ty: &'static str) {
                let mut listeners = super::LISTENERS_MAP.lock().unwrap();
                listeners.insert(elem_id, ty);
            }
        }

        impl $guest for $component {
            /// Say hello!
            fn render(name: String) -> String {
                let name = &name;

                if OUTPUT_ID.get().is_none() {
                    #[allow(clippy::redundant_closure)]
                    let id: &String = OUTPUT_ID.get_or_init(|| utils::rand_id());

                    // Render and return all HTML
                    html! {
                      <$page title={"CAN'T BE EVIL"}>
                        <$input name id={&utils::rand_id()} />
                        <$output name id />
                      </$page>
                    }
                } else {
                    // If OUTPUT_ID is set, render only the output section
                    // This is so we keep our INPUT event listeners which were set above
                    // Render and return only the output section of HTML
                    html! {
                      <$output name id={OUTPUT_ID.get().unwrap()} />
                    }
                }
            }

            /// Activate the component listeners
            fn activate() {
                let listeners = LISTENERS_MAP.lock().unwrap();
                for (selector, ty) in listeners.iter() {
                    let deets = $imports::ListenDetails {
                        selector: selector.to_string(),
                        ty: ty.to_string(),
                        value: "TODO".to_string(),
                    };

                    $imports::addeventlistener(&deets);
                }
            }
        }
    };
}
