use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Auth {
    pub method: String,
    pub path: String,
    pub tag: String,
    pub desc: String,
    pub middleware: String,
    pub auth: String,
}

impl Auth {
    #[allow(dead_code)]
    pub fn get_auth(&self) -> bool {
        if self.auth.eq("true") {
            return true;
        }
        return false;
    }
}

static AUTH_COLLECT: LazyLock<Mutex<HashMap<String, Auth>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

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

///
/// Examples
///```text
/// async fn test(req: actix_web::HttpRequest) -> impl Responder {
///     return throw!(&req, errcode::VALID_CODE_ERROR);
/// }
///```
///
#[macro_export]
#[allow(unused_macros)]
macro_rules! throw {
    ($a:expr, $b:expr) => {
        $b.throw($a)
    };
}

///
/// Examples
///```text
/// async fn test(req: actix_web::HttpRequest) -> impl Responder {
///     return throw_tips!(&req, errcode::VALID_CODE_ERROR, "kkk");
/// }
///```
///
#[macro_export]
#[allow(unused_macros)]
macro_rules! throw_tips {
    ($a:expr, $b:expr, $c:expr) => {
        $b.throw_tips($a, $c)
    };
}
