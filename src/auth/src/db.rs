use sqlx::{Pool, MySql, Error, MySqlPool};

pub async fn connect() -> Result<Pool<MySql>, Error> {
    return MySqlPool::connect("mysql://root:rey@localhost:3306/rustdb").await;
}