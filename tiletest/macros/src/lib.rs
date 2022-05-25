#[macro_use]
extern crate darling;
extern crate syn;

#[derive(Default, FromMeta)]
#[darling(default)]
pub struct Lorem {
    #[darling(rename = "sit")]
    ipsum: bool,
    dolor: Option<String>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(my_crate), forward_attrs(allow, doc, cfg))]
pub struct MyTraitOpts {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    lorem: Lorem,
}
