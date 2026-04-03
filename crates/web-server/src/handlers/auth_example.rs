use axum::Extension;
use toolcraft_axum_kit::{
    CommonResponse, IntoCommonResponse, ResponseResult, middleware::auth_mw::AuthUser,
};

use crate::dto::auth_example::CurrentUserResponse;

#[utoipa::path(
    get,
    path = "/me",
    tag = "auth-example",
    security(("Bearer" = [])),
    responses(
        (
            status = 200,
            description = "Current authenticated user",
            body = CommonResponse<CurrentUserResponse>
        ),
        (status = 401, description = "Unauthorized"),
    )
)]
pub async fn current_user(
    Extension(auth_user): Extension<AuthUser>,
) -> ResponseResult<CurrentUserResponse> {
    Ok(CurrentUserResponse {
        user_id: auth_user.user_id,
    }
    .into_common_response()
    .to_json())
}
