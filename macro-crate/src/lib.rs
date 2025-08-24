mod utils;
use proc_macro::TokenStream;
use quote::quote;
use syn::Lit;
use syn::Meta;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Data, Field, Fields, parse_macro_input};
use syn::{DeriveInput, Token};
use utils::{handle_length, handle_one_nv, rust_type_to_sql_type};

/// 处理表名
fn get_table_name(input: &DeriveInput) -> String {
    input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("table_name"))
        .and_then(|attr| {
            if let Meta::NameValue(meta) = &attr.meta {
                if let syn::Expr::Lit(expr_lit) = &meta.value {
                    if let Lit::Str(lit_str) = &expr_lit.lit {
                        return Some(lit_str.value());
                    }
                }
            }
            None
        })
        .unwrap_or_else(|| input.ident.to_string().to_lowercase())
}

/// 解析字段
fn parse_fields(fields: Punctuated<Field, Comma>, name: String) -> Vec<String> {
    let mut field_vec = Vec::new();
    let mut has_pk = false;
    for field in fields.into_iter() {
        let field_name = field.ident.unwrap();
        let ty = field.ty;
        let mut sql_type = rust_type_to_sql_type(&ty); // 让 sql_type 可变
        let mut sql_constraint = String::new();
        for attr in field
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("field"))
        {
            if let Meta::List(meta_list) = &attr.meta {
                let punctuated = Punctuated::<Meta, Token![,]>::parse_terminated;
                if let Ok(nested) = punctuated.parse2(meta_list.tokens.clone()) {
                    for meta in nested {
                        match meta {
                            Meta::Path(path) => {
                                if path.is_ident("pk") {
                                    has_pk = true;
                                    sql_constraint += " auto_increment primary key";
                                }
                            }
                            Meta::NameValue(nv) => {
                                let ident = nv.path.get_ident().unwrap().to_string();

                                if ident == "length" {
                                    // length 应该修改类型部分，而不是约束部分
                                    let length_value = handle_length(&nv.value);
                                    sql_type = format!("{}({})", sql_type, length_value);
                                } else {
                                    let res = handle_one_nv(nv);
                                    sql_constraint += &res;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        let field_sql = format!("{} {}{}", field_name, sql_type, sql_constraint);
        field_vec.push(field_sql);
    }
    if !has_pk {
        let pk_sql = format!("{}_id int auto_increment primary key", name);
        field_vec.push(pk_sql);
    }
    field_vec
}
#[proc_macro_derive(Create, attributes(table_name, field))]
pub fn create(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    //获取表名
    let table_name = get_table_name(&input);
    // 获取结构体的属性
    let fields = if let Data::Struct(s) = input.data {
        if let Fields::Named(n) = s.fields {
            n.named
        } else {
            panic!("不是命名结构体");
        }
    } else {
        panic!("不是结构体");
    };
    // sql属性和约束
    let word_vec = parse_fields(fields, table_name.clone());
    // 列
    let columns = word_vec.join(",\n ");
    // 拼接
    let output = quote! {
        impl #name {
            pub fn create_table_sql() -> String {
                format!(
                    "create table {} (\n{}\n);",
                    #table_name,
                    #columns
                )
            }
        }
    };
    output.into()
}
