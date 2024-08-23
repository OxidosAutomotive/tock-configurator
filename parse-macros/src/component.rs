// Copyright OxidOS Automotive 2024.

use darling::ast::NestedMeta;
use darling::{util::Flag, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::{format_ident, quote, ToTokens};
use syn::ItemStruct;

/// `peripheral` attribute proc macro arguments.
#[derive(Debug, FromMeta)]
struct PeripheralMacroArgs {
    peripherals: Option<String>,
    #[darling(flatten)]
    component_args: ComponentMacroArgs,
}

/// `component` attribute proc macro arguments.
#[derive(Debug, FromMeta)]
struct ComponentMacroArgs {
    serde: Flag,
    curr: Flag,
    ident: String,
}

/// Helper function used after parsing the macro attributes for generating the struct and the impl
/// blocks for the component.
fn define_struct(
    base_ident: String,
    curr: Flag,
    serde: Flag,
    peripherals: Option<String>,
    item: TokenStream,
) -> syn::Result<TokenStream> {
    // Get the struct definition.
    let struct_item =
        syn::parse::<ItemStruct>(item).map_err(|_| syn::Error::new(Span::call_site(), "AICI0!"))?;

    // Get the field that matches the field specified by the attribute.
    let fields = struct_item
        .fields
        .iter()
        .map(|f| f.to_token_stream())
        .collect::<Vec<_>>();

    // Retrieve information from the struct for redefining.
    let struct_ty = struct_item.ident;
    let struct_generics = struct_item.generics;

    let mut arg_num: usize = 0;
    let mut tuple_struct = false;

    let derive_default = if struct_item.fields.is_empty() {
        Some(quote!( #[derive(Default)] ))
    } else {
        None
    };

    // Get the identifiers and the types of the fields for the initialization function.
    let (field_names, field_types): (Vec<_>, Vec<_>) = struct_item
        .fields
        .iter()
        .map(|f| {
            match f.ident.clone() {
                Some(ident) => (ident.to_token_stream(), f.ty.clone()),
                None => {
                    let field = (
                        format_ident!("arg{}", arg_num.to_string()).to_token_stream(),
                        f.ty.clone(),
                    );
                    // This should be set only once but it's fine for now.
                    tuple_struct = true;
                    arg_num += 1;
                    field
                }
            }
        })
        .unzip();

    let struct_vis = struct_item.vis;

    // The crate's name is based on the `curr` flag.
    let _crate = if curr.is_present() {
        quote!(crate)
    } else {
        quote!(parse)
    };

    let (derive_serde, serde_skip) = serde
        .is_present()
        .then_some((
            quote!(#[derive(serde::Serialize, serde::Deserialize)]),
            quote!(#[serde(skip)]),
        ))
        .unzip();

    // Get generics and all.
    let (impl_generics, ty_generics, where_clause) = struct_generics.split_for_impl();

    if peripherals.is_some() {
        //  TODO: Implement PartialEq
    }

    let ident_init_expr = peripherals
        .map(|p_ident| format_ident!("{}", p_ident.clone()))
        .map(|ident| quote!(#ident.clone() + #base_ident))
        .unwrap_or(quote!( {
            use #_crate::FormatIdent;
            String::from(#base_ident) + &#_crate::Uuid::new_v4().format_ident()
        }));

    let arg_num_ident = Literal::usize_unsuffixed(arg_num);

    let (init_expr, self_ident, struct_ast) = if tuple_struct {
        (
            quote! (Self (#(#field_names,)* #ident_init_expr)),
            quote! (Ok(&self.#arg_num_ident)),
            quote! {
                #derive_default
                #derive_serde
                #struct_vis struct #struct_ty #struct_generics (#(#field_types,)*
                #serde_skip
                String);
            },
        )
    } else {
        (
            quote! {Self {
                 #(#field_names,)*
                __ident: #ident_init_expr
            }
            },
            quote!(Ok(&self.__ident)),
            quote! {
            #derive_default
            #derive_serde
            #struct_vis struct #struct_ty #struct_generics {
                #(#fields,)*
                #serde_skip
                __ident: String,
            }
            },
        )
    };

    // The final code generated with both the constructor and the `Ident` trait implementations.
    let impls = quote! {
        impl #impl_generics #struct_ty #ty_generics #where_clause {
            pub fn new(#(#field_names : #field_types),*) -> Self {
                #init_expr
            }
        }

        impl #impl_generics #_crate::component::Ident for #struct_ty #ty_generics #where_clause {
            fn ident(&self) -> Result<&str, #_crate::error::Error> {
                #self_ident
            }
        }
    };

    // The code generated with the constructor.
    Ok(quote!(
        #struct_ast
        #impls
    )
    .into())
}

/// Generate the expression for defining a component which is also a peripheral.
pub(crate) fn define_peripheral(attrs: TokenStream, item: TokenStream) -> TokenStream {
    // Get the dependency field from the attribute.
    let attr_args = match NestedMeta::parse_meta_list(attrs.into()) {
        Ok(v) => v,
        Err(err) => {
            return TokenStream::from(darling::Error::from(err).write_errors());
        }
    };

    let args = match PeripheralMacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(err) => {
            return TokenStream::from(err.write_errors());
        }
    };

    let peripherals_ident = args.peripherals.unwrap_or("PERIPHERALS".to_string());

    match define_struct(
        args.component_args.ident,
        args.component_args.curr,
        args.component_args.serde,
        Some(peripherals_ident),
        item,
    ) {
        Ok(tokens) => tokens,
        Err(err) => err.into_compile_error().into(),
    }
}

/// Generate the expression for defining a component.
pub(crate) fn define_component(attrs: TokenStream, item: TokenStream) -> TokenStream {
    // Get the dependency field from the attribute.
    let attr_args = match NestedMeta::parse_meta_list(attrs.into()) {
        Ok(v) => v,
        Err(err) => {
            return TokenStream::from(darling::Error::from(err).write_errors());
        }
    };

    let args = match PeripheralMacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(err) => {
            return TokenStream::from(err.write_errors());
        }
    };

    match define_struct(
        args.component_args.ident,
        args.component_args.curr,
        args.component_args.serde,
        None,
        item,
    ) {
        Ok(tokens) => tokens,
        Err(err) => err.into_compile_error().into(),
    }
}
