use crate::utils::ApiError;

pub async fn get_user_email(user_id:u64) -> Result<String, ApiError> {
    dbg!(user_id);
    if user_id == 1 {
        return Ok("test@test.com".to_string());
    }
    Err(ApiError{
        code:"not_found".into(),
        message: "user if not found".into()
    })
}