use axum::Extension;
use service::api::app::AppApi;
use toolcraft_axum_kit::{IntoCommonResponse, ResponseResult, middleware::auth_mw::AuthUser};

use crate::{
    dto::app::CurrentUserPermissionsResponse, error::from_biz_error,
    statics::db_manager::get_app_ctx,
};

pub async fn current_user_permissions(
    Extension(auth_user): Extension<AuthUser>,
) -> ResponseResult<CurrentUserPermissionsResponse> {
    let api = AppApi::new(get_app_ctx());
    let resp = api
        .get_current_user_permissions(auth_user.user_id)
        .await
        .map_err(from_biz_error)?;

    Ok(CurrentUserPermissionsResponse {
        user_id: resp.user_id,
        role_codes: resp.role_codes,
        permission_codes: resp.permission_codes,
    }
    .into_common_response()
    .to_json())
}
