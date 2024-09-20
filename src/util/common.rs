use crate::util::error::ErrorKind;
use crate::util::error::ToError;
use evolve_axum_cli::apis::configuration::{ApiKey, Configuration};
use fancy_regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use yew::virtual_dom::VNode;

pub type BasicResult<T, E = ErrorKind> = Result<T, E>;

const BASE_URL: &str = "http://localhost:3000";

pub fn validate_email(email: &str) -> BasicResult<()> {
    if email.is_empty() {
        return Err("please type in email".to_validation_error());
    }
    let reg = Regex::new(r#"\w[-\w.+]*@([A-Za-z0-9][-A-Za-z0-9]+\.)+[A-Za-z]{2,14}"#)?;
    if !reg.is_match(email)? {
        return Err("invalid email".to_validation_error());
    }
    Ok(())
}

pub fn validate_pwd(pwd: &str) -> BasicResult<()> {
    if pwd.is_empty() {
        return Err("please type in password".to_validation_error());
    }
    let reg = Regex::new(r#"(?=.*[a-z])(?=.*[0-9])[a-zA-Z0-9]{6,18}"#)?; //6位字母+数字,字母开头
    if !reg.is_match(pwd)? {
        return Err("invalid passowrd: length>=6, a-z and 0-9 is demanded".to_validation_error());
    }
    Ok(())
}

pub fn validate_pwd_confirm(pwd: &str, pwd_confirm: &str) -> BasicResult<()> {
    validate_pwd(pwd_confirm)?;
    if pwd != pwd_confirm {
        return Err("confirm password must as same as password".to_validation_error());
    }
    Ok(())
}

pub fn validate_code(code: &str) -> BasicResult<()> {
    if code.is_empty() {
        return Err("please type in code".to_validation_error());
    }
    let reg = Regex::new(r#"^\d{6}$"#)?; //6位字母+数字,字母开头
    if !reg.is_match(code)? {
        return Err("invalid code: length=6 and 0-9 is demanded".to_validation_error());
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CurrentUser {
    pub id: i64,
    pub r#type: String,
    pub email: String,
    pub name: Option<String>,
    pub mobile: Option<String>,
    pub laston: Option<i64>,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}

pub fn get_token() -> BasicResult<String> {
    let str = get_local_storage(crate::util::TOKEN_KEY)
        .ok_or(ErrorKind::OtherError(String::from("get token failed")))?;
    Ok(str)
}

pub fn get_cli_config_without_token() -> BasicResult<Configuration> {
    let mut ret = Configuration::default();
    ret.base_path = BASE_URL.to_string();
    Ok(ret)
}

pub fn get_cli_config() -> BasicResult<Configuration> {
    let mut ret = Configuration::default();
    ret.base_path = BASE_URL.to_string();
    ret.api_key = Some(ApiKey {
        prefix: None,
        key: get_token()?,
    });
    Ok(ret)
}

pub fn get_current_user() -> BasicResult<CurrentUser> {
    let str = get_local_storage("current_user").ok_or(ErrorKind::OtherError(String::from(
        "current user str is null",
    )))?;
    let res = serde_json::from_str::<CurrentUser>(&str)?;
    Ok(res)
}

pub fn delete_current_user() -> BasicResult<()> {
    del_local_storage(crate::util::TOKEN_KEY);
    del_local_storage("current_user");
    del_local_storage("selected_navbar_name");
    del_local_storage("selected_navbar_parent_name");
    Ok(())
}

pub fn set_local_storage(key: &str, value: &str) {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    local_storage.set_item(key, value).unwrap();
}

pub fn get_local_storage(key: &str) -> Option<String> {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    local_storage.get_item(key).unwrap()
}

pub fn del_local_storage(key: &str) {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    local_storage.delete(key).unwrap()
}

pub fn redirect(path: &str) {
    web_sys::window()
        .unwrap()
        .location()
        .set_pathname(path)
        .unwrap();
}

pub fn create_html(tag: &str, inner_html: &str) -> VNode {
    let element = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element(tag)
        .unwrap();
    element.set_inner_html(inner_html);
    let vnode = yew::virtual_dom::VNode::VRef(web_sys::Node::from(element));

    vnode
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum ValidStatus {
    Valid,
    InValid(String),
    None,
}

impl Default for ValidStatus {
    fn default() -> Self {
        ValidStatus::None
    }
}
