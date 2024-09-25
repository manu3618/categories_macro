extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(Category)]
pub fn derive_category(item: TokenStream) -> TokenStream {
    let name = item.into_iter().nth(1).unwrap(); // first 2 tokens should be `enum name`
    let new_func = [
        format!("impl {name}{{"),
        "pub fn categories() -> Vec<Self>{".into(),
        "Vec::new()".into(),
        "}".into(),
        "}".into(),
    ];
    new_func.join("\n").parse().unwrap()
}
