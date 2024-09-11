// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{
//   parse::{Parse, ParseStream},
//   parse_macro_input, Expr, Ident, ItemStruct, Token,
// };

// #[proc_macro_derive(Builder)]
// pub fn builder(input: TokenStream) -> TokenStream {
//   let _ast = parse_macro_input!(input as ItemStruct);

//   quote! {}.into()
// }

// struct MatchCfgInput {
//   arms: Vec<MatchArm>,
// }

// struct MatchArm {
//   platform: Ident,
//   body: Expr,
// }

// impl Parse for MatchCfgInput {
//   fn parse(input: ParseStream) -> syn::Result<Self> {
//     let mut arms = Vec::new();
//     while !input.is_empty() {
//       let platform: Ident = input.parse()?;
//       input.parse::<Token![=>]>()?;
//       let body: Expr = input.parse()?;
//       input.parse::<Token![,]>().ok();
//       arms.push(MatchArm { platform, body });
//     }
//     Ok(MatchCfgInput { arms })
//   }
// }

// #[proc_macro]
// pub fn matches_cfg(input: TokenStream) -> TokenStream {
//   let MatchCfgInput { arms } = parse_macro_input!(input as MatchCfgInput);

//   let mut expanded = proc_macro2::TokenStream::new();
//   let mut platforms = Vec::new();

//   for arm in arms {
//     let platform = arm.platform;
//     platforms.push(platform.clone());
//     let body = arm.body;
//     expanded.extend(quote! {
//       #[cfg(#platform)] {
//         return #body;
//       }
//     });
//   }

//   expanded.extend(quote! {
//     #[cfg(not(any(#(#platforms),*)))] {
//       panic!("No matching configuration found");
//     }
//   });

//   TokenStream::from(expanded)
// }
