extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{self, AttributeArgs, Data, DataStruct, DeriveInput, Fields, FieldsNamed};

// ! This mapping must be consistent with the one in `eParamType` enum
const UNIT_STATUS_ARRAY: [&str; 17] = [
    "hp",
    "atk",
    "def",
    "magic_str",
    "magic_def",
    "physical_critical",
    "magic_critical",
    "dodge",
    "life_steal",
    "wave_hp_recovery",
    "wave_energy_recovery",
    "physical_penetrate",
    "magic_penetrate",
    "energy_reduce_rate",
    "hp_recovery_rate",
    "energy_recovery_rate",
    "accuracy",
];

#[proc_macro_attribute]
pub fn return_as_is(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn impl_status(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(item as DeriveInput);
    let attr = syn::parse_macro_input!(_attr as AttributeArgs);

    match split_attr(attr) {
        Ok(attrs) => match impl_status_macro(attrs, &ast) {
            Ok(ts) => ts,
            Err(e) => e.to_compile_error().into(),
        },
        Err(e) => e.to_compile_error().into(),
    }
}

fn split_attr(attr: Vec<syn::NestedMeta>) -> syn::Result<Vec<(String, String)>> {
    let mut attrs = vec![];

    for attr in attr.into_iter() {
        match &attr {
            syn::NestedMeta::Lit(syn::Lit::Str(lit)) => {
                let lit = lit.value();
                let parts = lit.splitn(2, "{}").collect::<Vec<_>>();
                if parts.len() == 2 {
                    attrs.push((parts[0].to_string(), parts[1].to_string()))
                } else {
                    return Err(syn::Error::new_spanned(attr, "invalid attribute format"));
                }
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    attr,
                    "`#[status]` only accepts `#[status(\"value\")]`",
                ))
            }
        }
    }

    if attrs.is_empty() {
        attrs.push(("".to_string(), "".to_string()))
    }

    Ok(attrs)
}

fn impl_status_macro(
    attrs: Vec<(String, String)>,
    ast: &syn::DeriveInput,
) -> syn::Result<TokenStream> {
    match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => derive_status_from_named_struct(attrs, ast, named),
        Data::Struct(_) => Err(syn::Error::new_spanned(
            ast,
            "only named struct is supported",
        )),
        Data::Enum(_) => Err(syn::Error::new_spanned(ast, "enum is not supported")),
        Data::Union(_) => Err(syn::Error::new_spanned(ast, "union is not supported")),
    }
}

fn derive_status_from_named_struct(
    attrs: Vec<(String, String)>,
    ast: &DeriveInput,
    named: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> syn::Result<TokenStream> {
    let ident = &ast.ident;

    let funcs = attrs
        .into_iter()
        .filter_map(|(prefix, suffix)| {
            let hp_ident = format!("{}hp{}", prefix, suffix);
            let hp_field = named.iter().find(|x| match &x.ident {
                Some(ident) => *ident == hp_ident,
                None => false,
            });
            hp_field?;
            let hp_type = hp_field.unwrap().ty.clone();
            Some((prefix, hp_type, suffix))
        })
        .map(|(prefix, hp_type, suffix)| {
            let status = format_ident!("{}status{}", prefix, suffix);
            let params = UNIT_STATUS_ARRAY
            .into_iter()
            .map(|x| format_ident!("{}{}{}", prefix, x, suffix))
            .collect::<Vec<_>>();

            quote! {
                pub fn #status(&self) -> nalgebra::SVector<#hp_type, 17> {
                    nalgebra::vector![
                        #(self.#params),*
                    ]
                }
            }
        })
        .collect::<Vec<_>>();

    return Ok(quote! {
        #ast

        impl #ident {
            #(
                #funcs
            )*
        }
    }
    .into());
}
