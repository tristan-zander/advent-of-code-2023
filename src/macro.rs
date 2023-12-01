use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, ItemMod};

struct SolutionInput {
    pub solution_modules: Vec<ItemMod>,
}

impl Parse for SolutionInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut solution_modules = Vec::new();
        loop {
            if input.is_empty() {
                break;
            }
            let token: ItemMod = input.parse()?;
            solution_modules.push(token);
        }
        Ok(Self { solution_modules })
    }
}

#[proc_macro]
pub fn solutions(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as SolutionInput);

    let modules: proc_macro2::TokenStream = input
        .solution_modules
        .iter()
        .map(|m| m.ident.to_string())
        .enumerate()
        .map(|(i, ident)| {
            format!(
                "\"{}.1\" => crate::{}::part_one,\n\"{}.2\" => crate::{}::part_two",
                i + 1,
                ident,
                i + 1,
                ident
            )
        })
        .reduce(|acc, e| format!("{},\n{}", acc, e)).unwrap().parse().unwrap();


    let expanded = quote! {
        ::phf::phf_map! {
            #modules
        }
    };
    
    return TokenStream::from(expanded);
}
