# 解析代码
```rust
if let Meta::NameValue(meta)=&attr.meta{
                if let syn::Expr::Lit(expr_lit)=&meta.value{
                     if let Lit::Str(lit_str) = &expr_lit.lit {
                        return Some(lit_str.value());
                    }
                }
            }
```
这三个 `if let` 是模式匹配，逐层解析属性的结构：

1. `if let Meta::NameValue(meta) = &attr.meta`  
   检查属性是不是 `NameValue` 类型（如 `#[table_name = "xxx"]`）。

2. `if let syn::Expr::Lit(expr_lit) = &meta.value`  
   检查值是不是字面量表达式（如字符串字面量）。

3. `if let Lit::Str(lit_str) = &expr_lit.lit`  
   检查字面量是不是字符串类型，并获取其内容。

只有全部匹配成功，才返回字符串值，否则继续查找或返回默认值。
# if let Ok(Meta::List(MetaList { nested, .. })) = attr.parse_meta()  这什么意思
这句代码的意思是：  
尝试把属性 `attr` 解析为 `Meta::List` 类型（即形如 `#[field(...)]` 的属性），如果解析成功，就提取里面的参数列表 `nested`。  
常用于宏里处理带参数的属性，比如 `#[field(pk, null=false)]`。