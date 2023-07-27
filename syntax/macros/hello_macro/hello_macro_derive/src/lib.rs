// proc_macro这个crate是Rust自带的
use proc_macro::TokenStream;
// 引入quote这个crate将 syn 解析的数据结构转换回 Rust 代码
use quote::quote;
// 引入syn这个crate主要是将字符串中的 Rust 代码解析成为一个可以操作的数据结构
use syn;

// 这里为drive宏的主入口 HelloMacro为drive宏的名称
// 当在一个类型上指定 #[derive(HelloMacro)] 这个注解时，这里的hello_macro_derive函数就会被调用
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    // 第一步：负责解析TokenStream获得抽象语法树
    //        这个抽象语法树ast的类型是DeriveInput结构体
    // 注意：
    //       ❓这里为什么要调用unwrap方法
    //         当调用 syn::parse 函数失败时，用 unwrap 来使 hello_macro_derive 函数 panic。错误时 panic 对过程宏来说是必须的
    // 如果在一个叫做Pancakes结构体上定义了宏#[derive(HelloMacro)] ，此时解析出的DeriveInput可能是下面这个样子：
    /*
        DeriveInput {
            // --snip--

            ident: Ident {
                ident: "Pancakes",
                span: #0 bytes(95..103)
            },
            data: Struct(
                DataStruct {
                    struct_token: Struct,
                    fields: Unit,
                    semi_token: Some(
                        Semi
                    )
                }
            )
        }
     */
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    // 第二步：负责转换语法树，这里就是宏的核心实现逻辑
    impl_hello_macro(&ast)
}

// impl_hello_macro函数其用于构建所要包含在内的 Rust 新代码
// 使用解析过的 Rust 代码实现 HelloMacro trait
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // ast.ident 作为注解类型名字（标识符）的 Ident 结构体实例 
    // 如果在一个叫做Pancakes结构体上定义了宏#[derive(HelloMacro)]此时获取的ast.ident就是"Pancakes"
    let name = &ast.ident;
    // 这里的quote! 宏能让我们编写希望返回的 Rust 代码
    let gen = quote! {
        // 这里定义了宏需要生成的代码逻辑
        // 通过#name这里方式可以对代码模版进行相应的填充
        impl HelloMacro for #name {
            fn hello_macro() {
                // stringify! 为 Rust 内置宏，作用是其接收一个 Rust 表达式
                /*
                   如果接受的是 1 + 2 ， 然后在编译时将表达式转换为一个字符串常量，如 "1 + 2"
                   需要注意的是这与 format! 或 println! 是不同的，它计算表达式并将结果转换为 String 
                 */
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}