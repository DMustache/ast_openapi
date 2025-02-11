use syn::{visit::Visit, Expr, File, Item, ItemFn, PatIdent, Stmt};

fn main() {
    todo!()
}

pub fn find_main(ast: File) {
    let functions = ast
        .items
        .iter()
        .filter_map(|item| {
            if let Item::Fn(item_fn) = item {
                Some(item_fn.clone())
            } else {
                None
            }
        })
        .collect::<Vec<ItemFn>>();

    for func in functions {
        if &func.sig.ident == "main" {
            for statement in &func.block.stmts {
                if let Stmt::Local(local) = statement {
                    if let Some(pattern_ident) = get_pattern_ident(&local.pat) {
                        if pattern_ident.ident == "router" {
                            println!("{:?}", local.init.as_ref().map(|init| &*init.expr))
                        }
                    }
                }
            }
        }
    }
}

fn get_pattern_ident(pattern: &syn::Pat) -> Option<&PatIdent> {
    if let syn::Pat::Ident(pattern_ident) = pattern {
        return Some(pattern_ident);
    }
    None
}

#[derive(Debug)]
pub struct NestInfo {
    pub path: String,
    pub router: Expr,
}

// Visitor for scanning AST and finding Router functions
struct NestVisitor {
    nests: Vec<NestInfo>,
}

impl Visit<'_> for NestVisitor {
    fn visit_expr_method_call(&mut self, node: &'_ syn::ExprMethodCall) {
        todo!()
    }
}

#[test]
fn test_nest_parsing() {
    let code = std::fs::read_to_string("test_cases/basic.rs").unwrap();

    let ast = syn::parse_file(&code).unwrap();
    find_main(ast);
}
