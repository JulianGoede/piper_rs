extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Punct;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Ident, LitInt, LitStr};

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
        // eprintln!("{:?}", input);
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

#[proc_macro_attribute]
pub fn pipeline(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr_args as PipelineAttributes);
    let func = parse_macro_input!(item as proc_macro2::TokenStream);

    let name: Ident = Ident::new(&attr.name.value(), proc_macro2::Span::call_site());
    let retries: LitInt = attr.retries;
    let retry_delay_secs: LitInt = attr.retry_delay_secs;
    let cron = attr.cron.value();

    let generated_pipeline_code = quote!(
        use schema::Pipeline;

        pub struct #name {
            retries: u32,
            retry_delay_secs: u32,
            cron: String,
        }

        impl schema::Pipeline<Vec<String>> for #name {
            fn new() -> Self {
                #name {
                    retries: #retries,
                    retry_delay_secs: #retry_delay_secs,
                    cron: #cron.to_string()
                }
            }

            fn run(&self, args: &dyn std::any::Any) -> schema::RunResult<Vec<String>> {
                if let Some((ranking_url, day, unused_var)) = args.downcast_ref::<(String, String, u32)>() {
                    let results: Vec<String> = vec![ranking_url.to_string(), day.to_string()];
                    return Into::into(Ok(results));
                } else {
                    panic!("Unsupported arguments");
                }
            }

        }
    );

    let generated_tokens = quote!(
        #generated_pipeline_code

        #func
    );

    let generated_tokens = generated_tokens.into();
    generated_tokens
}
