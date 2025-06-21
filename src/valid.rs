use axum::extract::{FromRequest, FromRequestParts};
use axum::http::request::Parts;
use axum_valid::HasValidate;

use crate::error::ApiError;
use crate::param_valid::Query;



/**
* 完成校验失败时的异常转换
*/

#[derive(Debug, Clone, Default, FromRequest, FromRequestParts)]
#[from_request(via(axum_valid::Valid), rejection(ApiError))]
pub struct Valid<T>(pub T);


#[derive(Debug, Clone, Default)]
pub struct ValidQuery<T>(pub T);

impl<S, T> FromRequestParts<S> for ValidQuery<T> 
    where 
        S: Send + Sync,
        Query<T>: FromRequestParts<S> + HasValidate
{
    type Rejection = ApiError;

    #[doc = " Perform the extraction."]
    async fn from_request_parts(parts: &mut Parts,state: &S,) -> Result<Self,Self::Rejection> {
        let result = Valid::<Query<T?>>::from_request_parts(parts, state).await?;
        todo!()
    }
}
