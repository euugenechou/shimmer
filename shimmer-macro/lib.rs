use proc_macro::TokenStream;
use proc_macro2::{Group, Span, TokenStream as TokenStream2, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parse2, parse_macro_input, punctuated::Punctuated, token::Comma, DeriveInput, Expr, FnArg,
    Ident, ImplItem, ImplItemFn, ItemFn, ItemImpl, Pat,
};

#[proc_macro_attribute]
pub fn shimmer(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // AST o__o
    let ast = parse_macro_input!(input as DeriveInput);

    // Extract name and type.
    // TODO: Figure out how to extract impl'ed type.
    let ty = &ast.ident;
    let ident = Ident::new("SHIMMER_SHARED_STATE", Span::call_site());

    // Produce struct definition and its static initialization.
    // TODO: replace with `LazyCell` once it's stabilized.
    (quote! {
        #ast

        lazy_static::lazy_static!{
            static ref #ident: std::sync::Arc<std::sync::Mutex<#ty>> = std::sync::Arc::new(std::sync::Mutex::new(#ty::default()));
        }
    })
    .into()
}

#[proc_macro_attribute]
pub fn shimmer_hook(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemImpl);

    let impl_fns = extract_impl_fns(ast.clone());
    let hooks = build_hooks(impl_fns);

    (quote! {
        #(#hooks)*
    })
    .into()
}

fn extract_impl_fns(ast: ItemImpl) -> Vec<ImplItemFn> {
    ast.items
        .into_iter()
        .filter_map(|item| match item {
            ImplItem::Fn(impl_fn) => Some(impl_fn),
            _ => None,
        })
        .collect()
}

fn build_hooks(mut impl_fns: Vec<ImplItemFn>) -> Vec<TokenStream2> {
    // Remove receiver from the function arguments.
    impl_fns.iter_mut().for_each(|impl_fn| {
        let mut receiverless_inputs: Punctuated<FnArg, Comma> = Punctuated::new();

        impl_fn
            .sig
            .inputs
            .iter()
            .skip_while(|arg| matches!(arg, FnArg::Receiver(..)))
            .for_each(|arg| {
                receiverless_inputs.push(arg.clone());
            });

        impl_fn.sig.inputs = receiverless_inputs;
    });

    // Replace all receiver instances.
    let receiverless_impl_fns = impl_fns
        .clone()
        .into_iter()
        .map(|impl_fn| {
            impl_fn
                .to_token_stream()
                .into_iter()
                .map(|tree| receiver_replaced_tree(&tree))
                .collect::<TokenStream2>()
        })
        .collect::<Vec<_>>();

    // Prepare the actual hooks.
    receiverless_impl_fns
        .iter()
        .map(|hook| {
            let mut ast = parse2::<ItemFn>(hook.clone()).unwrap();

            // TODO: provide custom hooked function name.
            let sig = &ast.sig;
            let real = &sig.ident;
            let name = Ident::new(&format!("shimmer_hook_{}", &sig.ident), Span::call_site());

            // Filter out the argument names.
            let args = &sig
                .inputs
                .clone()
                .into_iter()
                .filter_map(|pair| match pair {
                    FnArg::Typed(pattern_type) => match *pattern_type.pat.clone() {
                        Pat::Ident(pat) => Some(pat.ident),
                        _ => None,
                    },
                    _ => None,
                })
                .collect::<Vec<_>>();

            // Generate actual call.
            let call = parse2::<Expr>(quote! {
                redhook::real!(#real)(#(#args,)*)
            })
            .unwrap();

            let hack = parse2::<Expr>(quote! {
                lazy_static::initialize(&SHIMMER_SHARED_STATE)
            })
            .unwrap();

            // Add hack and actual call to the block.
            ast.block.stmts.insert(0, syn::Stmt::Expr(hack, None));
            ast.block.stmts.push(syn::Stmt::Expr(call, None));
            let block = &ast.block;

            quote! {
                redhook::hook! {
                    #sig => #name #block
                }
            }
        })
        .collect()
}

fn receiver_replaced_tree(tree: &TokenTree) -> TokenStream2 {
    match tree {
        TokenTree::Group(group) => {
            let stream = group
                .stream()
                .into_iter()
                .map(|tree| receiver_replaced_tree(&tree));

            TokenTree::Group(Group::new(
                group.delimiter(),
                TokenStream2::from_iter(stream),
            ))
            .into()
        }
        TokenTree::Ident(ref ident) => {
            if &ident.clone().to_string() == "self" {
                quote! { SHIMMER_SHARED_STATE.lock().unwrap() }
            } else {
                tree.clone().into()
            }
        }
        _ => tree.clone().into(),
    }
}
