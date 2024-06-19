extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_derive(SingletonInstance)]
pub fn singleton_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_singleton_macro(&ast)
}

fn impl_singleton_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // Use std::sync::Once to ensure thread-safe initialization.
    let gen = quote! {
        static mut INSTANCE: Option<#name> = None;
        static INIT: std::sync::Once = std::sync::Once::new();

        impl #name {
            pub fn instance() -> &'static mut #name {
                unsafe {
                    INIT.call_once(|| {
                        let s = #name::initialize();
                        INSTANCE = Some(s);
                    });
                    INSTANCE.as_mut().unwrap()
                }
            }

            pub fn object_initialize(obj: #name) -> &'static mut #name {
                unsafe {
                    INSTANCE = Some(obj);
                    INSTANCE.as_mut().unwrap()
                }
            }

            pub fn object() -> &'static mut #name {
                unsafe {
                    INSTANCE.as_mut().unwrap()
                }
            }
        }
    };
    gen.into()
}