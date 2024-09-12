// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, Ident, ItemStruct};

// #[proc_macro_derive(Backend, attributes(window))]
// pub fn backend(input: TokenStream) -> TokenStream {
//   let item_struct = parse_macro_input!(input as ItemStruct);

//   let struct_span = item_struct.ident.span();
//   let backend = item_struct.ident;
//   // let window = item_struct.generics.type_params().next().unwrap();
//   let Some(attr) = item_struct.attrs.first() else {
//     return syn::Error::new(struct_span, "no attribute `Window` found for `CreateWindow`")
//       .into_compile_error()
//       .into();
//   };

//   let window = match attr.parse_args::<Ident>() {
//     Ok(window) => window,
//     Err(e) => return e.into_compile_error().into(),
//   };

//   quote! {
//     impl Backend for #backend {
//       fn create_window(&self, settings: ventana_hal::settings::WindowSettings) -> Box<dyn ventana_hal::Window> {
//         Box::new(#window::new(settings))
//       }
//     }
//   }
//   .into()
// }

// // struct MatchCfgInput {
// //   arms: Vec<MatchArm>,
// // }

// // struct MatchArm {
// //   platform: Ident,
// //   body: Expr,
// // }

// // impl Parse for MatchCfgInput {
// //   fn parse(input: ParseStream) -> syn::Result<Self> {
// //     let mut arms = Vec::new();
// //     while !input.is_empty() {
// //       let platform: Ident = input.parse()?;
// //       input.parse::<Token![=>]>()?;
// //       let body: Expr = input.parse()?;
// //       input.parse::<Token![,]>().ok();
// //       arms.push(MatchArm { platform, body });
// //     }
// //     Ok(MatchCfgInput { arms })
// //   }
// // }

// // #[proc_macro]
// // pub fn matches_cfg(input: TokenStream) -> TokenStream {
// //   let MatchCfgInput { arms } = parse_macro_input!(input as MatchCfgInput);

// //   let mut expanded = proc_macro2::TokenStream::new();
// //   let mut platforms = Vec::new();

// //   for arm in arms {
// //     let platform = arm.platform;
// //     platforms.push(platform.clone());
// //     let body = arm.body;
// //     expanded.extend(quote! {
// //       #[cfg(#platform)] {
// //         return #body;
// //       }
// //     });
// //   }

// //   expanded.extend(quote! {
// //     #[cfg(not(any(#(#platforms),*)))] {
// //       panic!("No matching configuration found");
// //     }
// //   });

// //   TokenStream::from(expanded)
// // }
