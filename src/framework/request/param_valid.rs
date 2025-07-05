use axum::extract::{FromRequest, FromRequestParts};
use axum_valid::HasValidate;

use super::super::error::ApiError;

/*
* 这里需要自定义抽取器，主要实现 FromRequestParts 到参数抽取
*/

// 处理去 query 参数错误

#[derive(Debug, Clone, Copy, Default, FromRequestParts)]
// 自定义的语法(宏规定的)
// 这句代码意义是将内部实现的 Query<T> 逻辑桥接到这里
// 同时将抛出的错误转变为 ApiError
#[from_request(via(axum::extract::Query), rejection(ApiError))]
pub struct Query<T>(pub T);

impl<T> HasValidate for Query<T> {
    type Validate = T;

    fn get_validate(&self) -> &Self::Validate {
        &self.0
    }
}


// 处理 json 的错误

#[derive(Debug, Clone, Default, FromRequest)]
#[from_request(via(axum::extract::Json), rejection(ApiError))]
pub struct Json<T>(pub T);

impl<T> HasValidate for Json<T> {
    type Validate = T;

    fn get_validate(&self) -> &Self::Validate {
        &self.0
    }
}

// 处理路径参数的错误

#[derive(Debug, Clone, Default, FromRequestParts)]
#[from_request(via(axum::extract::Path), rejection(ApiError))]
pub struct Path<T>(pub T);

impl<T> HasValidate for Path<T> {
    type Validate = T;

    fn get_validate(&self) -> &Self::Validate {
        &self.0
    }
}
