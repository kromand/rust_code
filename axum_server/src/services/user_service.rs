use crate::utils::ApiError;

pub async fn get_user_email(user_id:u64) -> Result<String, ApiError> {
    Ok("test@test.com".to_string())
}