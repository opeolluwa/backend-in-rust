use crate::{
    jwt::JwtClaims,
    shared::{ApiResponse, IntoApiResponse, ResponseBody},
};

pub async fn some_protected_resource(_: JwtClaims) -> ApiResponse<ResponseBody<&'static str>> {
    Ok(ApiResponse::from_parts("Shhhhhh! Top secret!", None))
}
