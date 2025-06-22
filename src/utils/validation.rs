
use std::{borrow::Cow, cell::LazyCell, collections::HashMap};

use regex::Regex;
use validator::ValidationError;


const MOBILE_PHONE_REGEX: LazyCell<Regex> = 
    LazyCell::new(|| Regex::new(r"^1[3-9]\d{9}$").expect("Fail compile mobile phone regex"));


pub fn is_mobile_phone(value: &str) -> Result<(), ValidationError> {
    if (MOBILE_PHONE_REGEX.is_match(value)) {
        Ok(())
    } else {
        Err(build_validation_error("手机号码格式不正确"))
    }
}

fn build_validation_error(message: &'static str) -> ValidationError {
    ValidationError {
        code: Cow::from("invalid"),
        message: Some(Cow::from(message)),
        params: HashMap::new()
    }
}
