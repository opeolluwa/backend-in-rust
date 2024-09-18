
use crate::{jwt::JwtClaims, shared::{ApiResponse, IntoApiResponse, ResponseBody}};

pub async fn some_protected_resources(_: JwtClaims) -> ApiResponse<ResponseBody<&'static str>> {
Ok(ApiResponse::from_parts("Shhhhhhhh! Top secret!", None))
}
