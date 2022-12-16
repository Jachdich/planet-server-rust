extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Lit, Meta, MetaNameValue};

#[proc_macro_derive(Tile)]
pub fn static_tile_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_static_tile(&ast)
}

fn impl_static_tile(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Tile for #name {
            fn get_type(&self) -> TileType {
                TileType::#name
            }
            fn tick(&self) -> Result<(), TileError> {
                #name::tick(&self)
            }
        }
    };
    gen.into()
}
