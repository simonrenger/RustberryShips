#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(EcsRetrievable)]
pub fn ecs_retrievable_derive(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_ecs_retrievable(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_ecs_retrievable(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl EcsRetrievable for #name {
            type T = #name;

            fn retrieve(ecs: &Ecs) -> Option<&Self::T>{
                let mut ids: Vec<EntityId> = Vec::new();
                let filter = component_filter!(#name);
                ecs.collect_with(&filter, &mut ids);
                if ids.is_empty() {
                    return None{};
                }
                let cmp: &Self::T = ecs.borrow(ids[0]).expect("We querried with ecs.collect_with but it is not there?");
                Some(cmp)
            }

            fn retrieve_mut(ecs: &mut Ecs) -> Option<&mut Self::T>{
                let mut ids: Vec<EntityId> = Vec::new();
                let filter = component_filter!(#name);
                ecs.collect_with(&filter, &mut ids);
                if ids.is_empty() {
                    return None{};
                }
                let cmp: &mut Self::T = ecs.borrow_mut(ids[0]).expect("We querried with ecs.collect_with but it is not there?");
                Some(cmp)
            }
        }
    }
}