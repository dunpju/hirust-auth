use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    pub path: String,
    pub tag: String,
    pub desc: String,
    pub middlewares: String,
    pub auth: bool,
}

static AUTH_COLLECT: LazyLock<Mutex<HashMap<String, Auth>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

type PrintFn = fn(t: &String, a: &Auth);

#[allow(unused)]
pub fn print(f: PrintFn) {
    for (tag, auth_info) in AUTH_COLLECT.lock().unwrap().iter() {
        f(tag, auth_info);
    }
}

#[allow(unused)]
pub fn insert(auth_info: Auth) {
    AUTH_COLLECT
        .lock()
        .unwrap()
        .insert(auth_info.tag.clone(), auth_info.clone());
}

#[allow(unused)]
pub fn get(tag: String) -> Auth {
    AUTH_COLLECT.lock().unwrap().get(&tag).unwrap().clone()
}

#[allow(unused)]
pub fn exist(tag: String) -> Option<Auth> {
    match AUTH_COLLECT.lock().unwrap().get(&tag) {
        Some(a) => Some(a.clone()),
        None => None,
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! unfold {
    ($scope:expr, $content:expr) => {
        $scope.configure($content)
        //$content
        /*let contents = include_str!("../.././public/scope").to_string();

        println!("contents {}", contents);*/

        // // 将字符串解析为Rust代码片段
        // let handler_expr = parse_str::<syn::Expr>(contents.as_str()).unwrap();
        // quote!(#handler_expr;).into();
    };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! throw {
    ($a:expr, $b:expr) => {
        $b.throw($a)
    };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! throw_tips {
    ($a:expr, $b:expr, $c:expr) => {
        $b.throw_tips($a, $c)
    };
}

// rust怎么获取文件名、行号、函数名 https://zhuanlan.zhihu.com/p/19832648722
#[macro_export]
#[allow(unused_macros)]
macro_rules! func {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}
