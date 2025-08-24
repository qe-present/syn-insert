use syn::Lit::{Bool, Int};
use syn::MetaNameValue;
use syn::{Expr, Lit};
use syn::Type;

pub fn rust_type_to_sql_type(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) if type_path.path.is_ident("i32") => "int".to_string(),
        Type::Path(type_path) if type_path.path.is_ident("f32") => "float".to_string(),
        Type::Path(type_path) if type_path.path.is_ident("String") => "varchar".to_string(),
        Type::Path(type_path) if type_path.path.is_ident("bool") => "tinyint".to_string(),
        _ => "".to_string(),
    }
}

fn handle_default(value: Expr) -> String {
    if let Expr::Lit(expr_lit) = &value {
        match &expr_lit.lit {
            Bool(lit_bool) => {
                return if lit_bool.value {
                    " default true".to_string()
                } else {
                    " default false".to_string()
                };
            }
            Lit::Str(lit_str) => {
                return format!(" default '{}'", lit_str.value());
            }
            Int(lit_int) => {
                return format!(" default {}", lit_int.base10_digits());
            }
            Lit::Float(lit_float) => {
                return format!(" default {}", lit_float.base10_digits());
            }
            _ => panic!("default 不支持该类型"),
        }
    }
    panic!("default 需要设为字面量值");
}
pub fn handle_length(value: &Expr) -> String {
    if let Expr::Lit(expr_lit) = &value {
        if let Int(lit_int) = &expr_lit.lit {
            return lit_int.base10_digits().to_string();
        }
    }
    panic!("需要设为整型");
}
pub fn handle_one_nv(nv: MetaNameValue) -> String {
    let path = nv.path;
    let value = nv.value;
    match path.get_ident().unwrap().to_string().as_str() {
        "null" => {
            if let Expr::Lit(expr_lit) = &value {
                if let Bool(lit_bool) = &expr_lit.lit {
                    return if lit_bool.value {
                        "".to_string()
                    } else {
                        " not null".to_string()
                    };
                }
            }
            panic!("null 需要设为bool值，例如: #[null = true]");
        }
        "default" => handle_default(value),
        _ => "".to_string(),
    }
}
