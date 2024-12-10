// src/setup.rs

use sea_orm::*;
use dotenv::dotenv;
use dotenvy_macro::dotenv;
// Replace with your database URL and database name



pub async fn set_up_db() -> Result<DatabaseConnection, DbErr> {

    dotenv().ok();
    // const PORT:&str = dotenv!("PORT");


    const DATABASE_URL: &str = dotenv!("DATABASE_URL") ;
    const DB_NAME: &str =  dotenv!("DB_NAME") ;

    let db = Database::connect(DATABASE_URL).await?;

    let db = match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
            ))
            .await?;

            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", DB_NAME),
            ))
            .await?;

            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };

    Ok(db)
}