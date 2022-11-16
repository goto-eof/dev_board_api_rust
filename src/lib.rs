use proc_macro::{Span, TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn add_permission(_: TokenStream, item: TokenStream) -> TokenStream {
    // let cloned = item.clone();
    let mut input = syn::parse_macro_input!(item as syn::ItemFn);

    let fn_name = input.sig.ident.to_string();
    let source = file!();
    let permission_name = format!("{}:{}", source, fn_name);
    println!("Permission: {}", permission_name);

    let output = quote::quote! {
        #input
    };
    output.into()
}

#[proc_macro_attribute]
pub fn is_authenticated(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut prefix: TokenStream = "
      println!(\"yahoo2 {:?} {:?}\",
          file!(), module_path!());
    "
    .parse()
    .unwrap();
    item.into_iter()
        .map(|tt| {
            match tt {
          TokenTree::Group(ref g) // match function body
              if g.delimiter() == proc_macro::Delimiter::Brace => {

                  // add logic before function body
                  prefix.extend(g.stream());

                  // return new function body as TokenTree
                  TokenTree::Group(proc_macro::Group::new(
                      proc_macro::Delimiter::Brace, prefix.clone()))
          },
          other => other, // else just forward
      }
        })
        .collect()
}
