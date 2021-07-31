pub fn get_ks_source(ast: &syn::DeriveInput) -> Option<String> {
    for attr in &ast.attrs[..] {
        if attr.path.segments[0].ident == "kaitai_source" {
            let tokens: syn::Expr = syn::parse2(attr.tokens.clone()).unwrap();
            if let syn::Expr::Paren(syn::ExprParen { expr: e, .. }) = tokens {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(ref s),
                    ..
                }) = *e
                {
                    return Some(s.value());
                }
            }
        }
    }
    None
}
