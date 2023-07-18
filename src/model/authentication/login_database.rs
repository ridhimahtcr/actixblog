use serde::Deserialize;
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Pool, Postgres, Row};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct LoginTest {
    pub(crate) value: i64,
}

impl<'r> FromRow<'r, PgRow> for LoginTest {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let name = row.try_get("count")?;
        Ok(LoginTest { value: name })
    }
}

pub async fn login_database(
    user: &String,
    password: String,
    db: &Pool<Postgres>,
) -> Result<LoginTest, anyhow::Error> {
    let login =
        sqlx::query_as::<_, LoginTest>("select count(1) from users where name=$1 AND password=$2")
            .bind(user)
            .bind(password)
            .fetch_one(db)
            .await?;

    Ok(login)
}
