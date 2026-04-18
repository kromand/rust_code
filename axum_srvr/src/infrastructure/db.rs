use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use crate::dto::user_dto::CreateUser;
use crate::dto::user_dto::User;


pub async fn init_db_pool(database_url: &str) -> Result<sqlx::PgPool,sqlx::Error>{
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(database_url)
        .await?;
    Ok(pool)
}

pub async fn create_user_db(pool: &PgPool, user_id: i32, data:&CreateUser) -> Result<i32,sqlx::Error>
{
    let record = sqlx::query!(
        r#"
        INSERT INTO generic (id,name,data)
        VALUES($1,$2,$3)
        RETURNING id
        "#,
        user_id,
        data.username,
        data.email
    )
    .fetch_one(pool)
    .await?;
    Ok(record.id)
}

pub async fn get_user_by_id(pool: &PgPool, user_id: i32) -> Result<User,sqlx::Error>
{
    let row = sqlx::query_as!(
        User,
        r#"
        SELECT id, name,data
        FROM generic
        WHERE id = $1
        "#,
        user_id)
        .fetch_one(pool)
        .await?;

    Ok(row)
}

pub async fn update_user_data(pool: &PgPool, user_id: i32, new_data:String) -> Result<bool,sqlx::Error>
{
    let result = sqlx::query!(
        r#"
        UPDATE generic
        SET data = $1
        WHERE id = $2
        "#,
        new_data,
        user_id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0{
        return Ok(false);
    }
    Ok(true)

}

pub async fn delete_user(pool: &PgPool, user_id: i32) -> Result<bool,sqlx::Error>
{
    let result = sqlx::query!(
        r#"
        DELETE FROM generic
        WHERE id = $1
        "#,
        user_id)
        .execute(pool)
        .await?;
    
    if result.rows_affected() == 0{
        return Ok(false);
    }
    Ok(true)
}