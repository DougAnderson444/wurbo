// Generated by `wit-bindgen` 0.21.0. DO NOT EDIT!
// Options used:
pub mod demo {
    pub mod vowels {
        #[allow(clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            /// Details required in order to add an event listener to an element
            #[derive(Clone)]
            pub struct ListenDetails {
                pub selector: _rt::String,
                pub ty: _rt::String,
            }
            impl ::core::fmt::Debug for ListenDetails {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("ListenDetails")
                        .field("selector", &self.selector)
                        .field("ty", &self.ty)
                        .finish()
                }
            }
            /// Context for the minijinja rendering
            #[derive(Clone)]
            pub struct Page {
                pub title: _rt::String,
            }
            impl ::core::fmt::Debug for Page {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("Page").field("title", &self.title).finish()
                }
            }
            #[derive(Clone)]
            pub struct Input {
                pub placeholder: _rt::String,
            }
            impl ::core::fmt::Debug for Input {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("Input")
                        .field("placeholder", &self.placeholder)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct Output {
                pub value: _rt::String,
                /// optional id string: None is intial render, Some for update value
                pub id: Option<_rt::String>,
                pub template: Option<_rt::String>,
            }
            impl ::core::fmt::Debug for Output {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("Output")
                        .field("value", &self.value)
                        .field("id", &self.id)
                        .field("template", &self.template)
                        .finish()
                }
            }
            /// COntent for the entire page
            #[derive(Clone)]
            pub struct Content {
                pub page: Page,
                pub input: Input,
                pub output: Option<Output>,
            }
            impl ::core::fmt::Debug for Content {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("Content")
                        .field("page", &self.page)
                        .field("input", &self.input)
                        .field("output", &self.output)
                        .finish()
                }
            }
            /// Context variants
            #[derive(Clone)]
            pub enum Context {
                AllContent(Content),
                Phrase(_rt::String),
            }
            impl ::core::fmt::Debug for Context {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        Context::AllContent(e) => {
                            f.debug_tuple("Context::AllContent").field(e).finish()
                        }
                        Context::Phrase(e) => f.debug_tuple("Context::Phrase").field(e).finish(),
                    }
                }
            }
        }

        #[allow(clippy::all)]
        pub mod wurbo_in {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            pub type ListenDetails = super::super::super::demo::vowels::types::ListenDetails;
            #[allow(unused_unsafe, clippy::all)]
            /// Add an event listener to the given element
            pub fn addeventlistener(details: &ListenDetails) {
                unsafe {
                    let super::super::super::demo::vowels::types::ListenDetails {
                        selector: selector0,
                        ty: ty0,
                    } = details;
                    let vec1 = selector0;
                    let ptr1 = vec1.as_ptr().cast::<u8>();
                    let len1 = vec1.len();
                    let vec2 = ty0;
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();

                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "demo:vowels/wurbo-in")]
                    extern "C" {
                        #[link_name = "addeventlistener"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8, _: usize);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8, _: usize) {
                        unreachable!()
                    }
                    wit_import(ptr1.cast_mut(), len1, ptr2.cast_mut(), len2);
                }
            }
        }
    }
}
pub mod exports {
    pub mod demo {
        pub mod vowels {
            #[allow(clippy::all)]
            pub mod wurbo_out {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Context = super::super::super::super::demo::vowels::types::Context;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_customize_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    let base6 = arg0;
                    let len6 = arg1;
                    let mut result6 = _rt::Vec::with_capacity(len6);
                    for i in 0..len6 {
                        let base = base6.add(i * 16);
                        let e6 = {
                            let l0 = *base.add(0).cast::<*mut u8>();
                            let l1 = *base.add(4).cast::<usize>();
                            let len2 = l1;
                            let bytes2 = _rt::Vec::from_raw_parts(l0.cast(), len2, len2);
                            let l3 = *base.add(8).cast::<*mut u8>();
                            let l4 = *base.add(12).cast::<usize>();
                            let len5 = l4;
                            let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);

                            (_rt::string_lift(bytes2), _rt::string_lift(bytes5))
                        };
                        result6.push(e6);
                    }
                    _rt::cabi_dealloc(base6, len6 * 16, 4);
                    let result7 = T::customize(result6);
                    let ptr8 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result7 {
                        Ok(_) => {
                            *ptr8.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr8.add(0).cast::<u8>() = (1i32) as u8;
                            let vec9 = (e.into_bytes()).into_boxed_slice();
                            let ptr9 = vec9.as_ptr().cast::<u8>();
                            let len9 = vec9.len();
                            ::core::mem::forget(vec9);
                            *ptr8.add(8).cast::<usize>() = len9;
                            *ptr8.add(4).cast::<*mut u8>() = ptr9.cast_mut();
                        }
                    };
                    ptr8
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_customize<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => (),
                        _ => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_render_cabi<T: Guest>(
                    arg0: i32,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                    arg5: i32,
                    arg6: *mut u8,
                    arg7: usize,
                    arg8: i32,
                    arg9: *mut u8,
                    arg10: usize,
                    arg11: i32,
                    arg12: *mut u8,
                    arg13: usize,
                ) -> *mut u8 {
                    use super::super::super::super::demo::vowels::types::Context as V6;
                    let v6 = match arg0 {
                        0 => {
                            let e6 = {
                                let len0 = arg2;
                                let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                                let len1 = arg4;
                                let bytes1 = _rt::Vec::from_raw_parts(arg3.cast(), len1, len1);

                                super::super::super::super::demo::vowels::types::Content {
                                    page: super::super::super::super::demo::vowels::types::Page {
                                        title: _rt::string_lift(bytes0),
                                    },
                                    input: super::super::super::super::demo::vowels::types::Input {
                                        placeholder: _rt::string_lift(bytes1),
                                    },
                                    output: match arg5 {
                                        0 => None,
                                        1 => {
                                            let e = {
                                                let len2 = arg7;
                                                let bytes2 = _rt::Vec::from_raw_parts(
                                                    arg6.cast(),
                                                    len2,
                                                    len2,
                                                );

                                                super::super::super::super::demo::vowels::types::Output{
                      value: _rt::string_lift(bytes2),
                      id: match arg8 {
                        0 => None,
                        1 => {
                          let e = {
                            let len3 = arg10;
                            let bytes3 = _rt::Vec::from_raw_parts(arg9.cast(), len3, len3);

                            _rt::string_lift(bytes3)
                          };
                          Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                      },
                      template: match arg11 {
                        0 => None,
                        1 => {
                          let e = {
                            let len4 = arg13;
                            let bytes4 = _rt::Vec::from_raw_parts(arg12.cast(), len4, len4);

                            _rt::string_lift(bytes4)
                          };
                          Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                      },
                    }
                                            };
                                            Some(e)
                                        }
                                        _ => _rt::invalid_enum_discriminant(),
                                    },
                                }
                            };
                            V6::AllContent(e6)
                        }
                        n => {
                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                            let e6 = {
                                let len5 = arg2;
                                let bytes5 = _rt::Vec::from_raw_parts(arg1.cast(), len5, len5);

                                _rt::string_lift(bytes5)
                            };
                            V6::Phrase(e6)
                        }
                    };
                    let result7 = T::render(v6);
                    let ptr8 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result7 {
                        Ok(e) => {
                            *ptr8.add(0).cast::<u8>() = (0i32) as u8;
                            let vec9 = (e.into_bytes()).into_boxed_slice();
                            let ptr9 = vec9.as_ptr().cast::<u8>();
                            let len9 = vec9.len();
                            ::core::mem::forget(vec9);
                            *ptr8.add(8).cast::<usize>() = len9;
                            *ptr8.add(4).cast::<*mut u8>() = ptr9.cast_mut();
                        }
                        Err(e) => {
                            *ptr8.add(0).cast::<u8>() = (1i32) as u8;
                            let vec10 = (e.into_bytes()).into_boxed_slice();
                            let ptr10 = vec10.as_ptr().cast::<u8>();
                            let len10 = vec10.len();
                            ::core::mem::forget(vec10);
                            *ptr8.add(8).cast::<usize>() = len10;
                            *ptr8.add(4).cast::<*mut u8>() = ptr10.cast_mut();
                        }
                    };
                    ptr8
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_render<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                        _ => {
                            let l3 = *arg0.add(4).cast::<*mut u8>();
                            let l4 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l3, l4, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_activate_cabi<T: Guest>(
                    arg0: i32,
                    arg1: *mut u8,
                    arg2: usize,
                ) {
                    T::activate(match arg0 {
                        0 => None,
                        1 => {
                            let e = {
                                let base3 = arg1;
                                let len3 = arg2;
                                let mut result3 = _rt::Vec::with_capacity(len3);
                                for i in 0..len3 {
                                    let base = base3.add(i * 8);
                                    let e3 = {
                                        let l0 = *base.add(0).cast::<*mut u8>();
                                        let l1 = *base.add(4).cast::<usize>();
                                        let len2 = l1;
                                        let bytes2 =
                                            _rt::Vec::from_raw_parts(l0.cast(), len2, len2);

                                        _rt::string_lift(bytes2)
                                    };
                                    result3.push(e3);
                                }
                                _rt::cabi_dealloc(base3, len3 * 8, 4);

                                result3
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    });
                }
                pub trait Guest {
                    /// Optionally customize the configuration of the templates used to render the component
                    fn customize(
                        templates: _rt::Vec<(_rt::String, _rt::String)>,
                    ) -> Result<(), _rt::String>;
                    /// renders the initial Web component with the given data
                    fn render(ctx: Context) -> Result<_rt::String, _rt::String>;
                    /// activate listening
                    fn activate(selectors: Option<_rt::Vec<_rt::String>>);
                }
                #[doc(hidden)]

                macro_rules! __export_demo_vowels_wurbo_out_cabi{
    ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

      #[export_name = "demo:vowels/wurbo-out#customize"]
      unsafe extern "C" fn export_customize(arg0: *mut u8,arg1: usize,) -> *mut u8 {
        $($path_to_types)*::_export_customize_cabi::<$ty>(arg0, arg1)
      }
      #[export_name = "cabi_post_demo:vowels/wurbo-out#customize"]
      unsafe extern "C" fn _post_return_customize(arg0: *mut u8,) {
        $($path_to_types)*::__post_return_customize::<$ty>(arg0)
      }
      #[export_name = "demo:vowels/wurbo-out#render"]
      unsafe extern "C" fn export_render(arg0: i32,arg1: *mut u8,arg2: usize,arg3: *mut u8,arg4: usize,arg5: i32,arg6: *mut u8,arg7: usize,arg8: i32,arg9: *mut u8,arg10: usize,arg11: i32,arg12: *mut u8,arg13: usize,) -> *mut u8 {
        $($path_to_types)*::_export_render_cabi::<$ty>(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13)
      }
      #[export_name = "cabi_post_demo:vowels/wurbo-out#render"]
      unsafe extern "C" fn _post_return_render(arg0: *mut u8,) {
        $($path_to_types)*::__post_return_render::<$ty>(arg0)
      }
      #[export_name = "demo:vowels/wurbo-out#activate"]
      unsafe extern "C" fn export_activate(arg0: i32,arg1: *mut u8,arg2: usize,) {
        $($path_to_types)*::_export_activate_cabi::<$ty>(arg0, arg1, arg2)
      }
    };);
  }
                #[doc(hidden)]
                pub(crate) use __export_demo_vowels_wurbo_out_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 12]);
                static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 12]);
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr as *mut u8, layout);
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_main_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::demo::vowels::wurbo_out::__export_demo_vowels_wurbo_out_cabi!($ty with_types_in $($path_to_types_root)*::exports::demo::vowels::wurbo_out);
  )
}
#[doc(inline)]
pub(crate) use __export_main_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.21.0:main:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 650] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x8f\x04\x01A\x02\x01\
A\x08\x01B\x0e\x01r\x02\x08selectors\x02tys\x04\0\x0elisten-details\x03\0\0\x01r\
\x01\x05titles\x04\0\x04page\x03\0\x02\x01r\x01\x0bplaceholders\x04\0\x05input\x03\
\0\x04\x01ks\x01r\x03\x05values\x02id\x06\x08template\x06\x04\0\x06output\x03\0\x07\
\x01k\x08\x01r\x03\x04page\x03\x05input\x05\x06output\x09\x04\0\x07content\x03\0\
\x0a\x01q\x02\x0ball-content\x01\x0b\0\x06phrase\x01s\0\x04\0\x07context\x03\0\x0c\
\x03\x01\x11demo:vowels/types\x05\0\x02\x03\0\0\x0elisten-details\x01B\x04\x02\x03\
\x02\x01\x01\x04\0\x0elisten-details\x03\0\0\x01@\x01\x07details\x01\x01\0\x04\0\
\x10addeventlistener\x01\x02\x03\x01\x14demo:vowels/wurbo-in\x05\x02\x02\x03\0\0\
\x07context\x01B\x0e\x02\x03\x02\x01\x03\x04\0\x07context\x03\0\0\x01o\x02ss\x01\
p\x02\x01j\0\x01s\x01@\x01\x09templates\x03\0\x04\x04\0\x09customize\x01\x05\x01\
j\x01s\x01s\x01@\x01\x03ctx\x01\0\x06\x04\0\x06render\x01\x07\x01ps\x01k\x08\x01\
@\x01\x09selectors\x09\x01\0\x04\0\x08activate\x01\x0a\x04\x01\x15demo:vowels/wu\
rbo-out\x05\x04\x04\x01\x10demo:vowels/main\x04\0\x0b\x0a\x01\0\x04main\x03\0\0\0\
G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.201.0\x10wit-bindge\
n-rust\x060.21.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
