extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Punct;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{
    parse_macro_input, GenericArgument, Ident, ItemFn, LitInt, LitStr, Path, PathArguments, Type,
    TypePath,
};

#[allow(dead_code)]
struct PipelineAttributes {
    name: LitStr,
    retries: LitInt,
    retry_delay_secs: LitInt,
    cron: LitStr,
}

fn parse_attribute<T: Parse>(input: ParseStream, key_name: String) -> syn::Result<T> {
    let lookahead = input.lookahead1();
    if lookahead.peek(Ident) {
        let fork = input.fork();
        let key: Ident = fork.parse()?;
        if key != key_name {
            let err_msg = format!("Expected attribute `{}` but got `{}`", key_name, key);
            return Err(input.error(err_msg));
        }
        let _ = input.parse::<Ident>();
    } else {
        let _: Punct = input.parse()?;
        let fork = input.fork();
        let key: Ident = fork.parse()?;
        if key != key_name {
            let err_msg = format!("Expected attribute `{}` but got `{}`", key_name, key);
            return Err(input.error(err_msg));
        }
        let _ = input.parse::<Ident>();
    }
    let _: Punct = input.parse()?;
    let _: Punct = input.parse()?;
    match input.parse() {
        Ok(value) => Ok(value),
        Err(e) => {
            let message = format!("Invalid value for `{}`: `{}`", key_name, e);
            Err(input.error(message))
        }
    }
}

impl Parse for PipelineAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = parse_attribute(input, "name".to_string())?;
        let retries = parse_attribute(input, "retries".to_string())?;
        let retry_delay_secs = parse_attribute(input, "retry_delay_secs".to_string())?;
        let cron = parse_attribute(input, "cron".to_string())?;
        Ok(PipelineAttributes {
            name,
            retries,
            retry_delay_secs,
            cron,
        })
    }
}

fn get_typed_fn_args<'a>(
    sig: &'a syn::Signature,
) -> (Vec<&'a proc_macro2::Ident>, Vec<&'a syn::Type>) {
    sig.inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Typed(typed_arg) => {
                if let syn::Pat::Ident(ident) = typed_arg.pat.as_ref() {
                    return (&ident.ident, typed_arg.ty.as_ref());
                } else {
                    panic!("Only named arguments are supported for pipeline");
                }
            }
            syn::FnArg::Receiver(_) => {
                panic!("functions with self args are not supported for pipeline")
            }
        })
        .unzip()
}

// returns (T, None) for a function foo(..) -> T
// and (T, E) for a function foo(..) -> Result<T, E>
fn infer_return_type(
    fn_return_type: &syn::ReturnType,
) -> (proc_macro2::TokenStream, Option<proc_macro2::TokenStream>) {
    match fn_return_type {
        syn::ReturnType::Default => (quote!(()), None),
        syn::ReturnType::Type(_, ty) => {
            if let Type::Path(TypePath {
                path: Path { segments, .. },
                ..
            }) = ty.as_ref()
            {
                if let Some(segment) = segments.last() {
                    if segment.ident == "Result" {
                        if let PathArguments::AngleBracketed(result_args) = &segment.arguments {
                            let result_types: Vec<proc_macro2::TokenStream> = result_args
                                .args
                                .iter()
                                .filter_map(|arg| {
                                    if let GenericArgument::Type(result_type) = arg {
                                        Some(quote!(#result_type))
                                    } else {
                                        None
                                    }
                                })
                                .collect();

                            return match result_types.as_slice() {
                                [t, e, ..] => (quote!(#t), Some(quote!(#e))),
                                [t] => (quote!(#t), None),
                                [] => (quote!(#ty), None),
                            };
                        }

                        return (quote!(#ty), None);
                    }
                }
                return (quote!(#ty), None);
            }
            (quote!(#ty), None)
        }
    }
}

#[proc_macro_attribute]
pub fn pipeline(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr_args as PipelineAttributes);
    let func = parse_macro_input!(item as ItemFn);

    let fn_name = &func.sig.ident;
    let (arg_names, arg_types): (Vec<&proc_macro2::Ident>, Vec<&syn::Type>) =
        get_typed_fn_args(&func.sig);

    let name: Ident = Ident::new(&attr.name.value(), proc_macro2::Span::call_site());
    let retries: LitInt = attr.retries;
    let retry_delay_secs: LitInt = attr.retry_delay_secs;
    let cron = attr.cron.value();

    let (success_type, maybe_error_type) = infer_return_type(&func.sig.output);
    let ty = match maybe_error_type {
        Some(error_type) => quote!(#success_type, #error_type),
        None => quote!(#success_type),
    };
    let pipeline_schema = quote!(schema::Pipeline<#ty>);
    let run_signature = quote!(fn run(&mut self, args: &dyn ::std::any::Any) -> schema::RunResult<#ty>);

    let generated_pipeline_code = quote!(
        #func

        pub struct #name {
            retries: u32,
            retry_delay_secs: u32,
            cron: String,
        }

        impl #pipeline_schema for #name {
            fn new() -> Self {
                #name {
                    retries: #retries,
                    retry_delay_secs: #retry_delay_secs,
                    cron: #cron.to_string()
                }
            }

            #run_signature {
                if let Some((#(#arg_names),*)) = args.downcast_ref::<(#(#arg_types),*)>() {
                    let results = #fn_name(#(#arg_names.to_owned()),*);
                    match schema::RunResult::from(results) {
                        schema::RunResult::Ok(t) => return schema::RunResult::Ok(t),
                        schema::RunResult::Retry(e) | schema::RunResult::Err(e) => {
                            if self.retries > 0 {
                                self.retries = self.retries -1;
                                return schema::RunResult::Retry(e);
                            }
                            return schema::RunResult::Err(e);
                        }
                    }
                } else {
                    panic!("Unsupported arguments");
                }
            }

        }
    );

    let generated_tokens = generated_pipeline_code.into();
    generated_tokens
}
