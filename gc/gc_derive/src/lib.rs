use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Trace)]
pub fn trace_derive(def: TokenStream) -> TokenStream {
    let def: syn::ItemStruct = syn::parse(def).unwrap();
    let struct_name = def.ident;
    let syn::Fields::Named(fields) = def.fields else {
        todo!("implement macro for tuple structs");
    };

    let trace_fields = fields.named.iter().map(|x| {
        let x = &x.ident;
        quote!{
            self.#x.trace(m);
        }
    });

    quote! {
        unsafe impl ::gc::Trace for #struct_name {
            fn trace(&self, m: &mut ::gc::Marker) {
                #(#trace_fields)*
            }
        }
    }.into()
}
