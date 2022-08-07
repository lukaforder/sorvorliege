use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input, ItemFn, AttributeArgs};

#[proc_macro_attribute]
pub fn command(attrs: TokenStream, stream: TokenStream) -> TokenStream {
  let input = parse_macro_input!(stream as ItemFn);
  let name = parse_macro_input!(attrs as AttributeArgs);
  let name = name.first().unwrap();

  let ItemFn {attrs, vis, sig, block} = input;
  let stmts = &block.stmts;
  let t = quote! {
    #(#attrs)* #vis #sig {
        let mut #name: Vec<tungstenite::Message> = Vec::new();
        #(#stmts)*
        #name
    }
  };
  t.into()
}