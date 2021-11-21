use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_str};

/// Parodies the `null-forgiving` syntax
///
/// # Examples
///
/// ```
/// // From
/// none_forgiving! {
///     fn strong_convert(str: &str) -> i32 {
///         str.parse::<f32>()!.to_string().parse()!
///     }
/// }
///
/// // To
/// fn strong_convert(str: &str) -> i32 {
///     str.parse::<f32>().unwrap().to_string().parse().unwrap()
/// }
/// ```
#[proc_macro]
pub fn none_forgiving(input: TokenStream) -> TokenStream {
    let mut input = input.to_string();
    let input = input.replace("!.", ".unwrap().");
    let input = input.replace("!;", ".unwrap();");
    let input = input.replace("! }", ".unwrap() }");
    input.parse().unwrap()
}
