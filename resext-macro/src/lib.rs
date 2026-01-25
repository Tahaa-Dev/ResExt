use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn resext(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item // Passthrough for now
}
