use std::time::Duration;
use s3::{creds::Credentials, Bucket, Region};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Debug)]
pub struct Config {
    pub pool: PgPool,
    pub address: String,
    pub jwt: String,
    pub s3: Box<Bucket>,
}

pub async fn init_config() -> Config {
    dotenvy::dotenv().expect("Unable to access .env file");

    let server_address = std::env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS not found in env file");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env file");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not found in env file");

    let s3_endpoint = std::env::var("S3_ENDPOINT").expect("S3_ENDPOINT not found in env file");
    let s3_access_key = std::env::var("S3_ACCESS_KEY").expect("S3_ACCESS_KEY not found in env file");
    let s3_secret_key = std::env::var("S3_SECRET_KEY").expect("S3_SECRET_KEY not found in env file");
    let s3_bucket = std::env::var("S3_BUCKET").expect("S3_BUCKET not found in env file");

    let db_pool = PgPoolOptions::new()
        .max_connections(64)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("Can't connect to database");

    let region = Region::Custom {
        region: "".to_owned(),
        endpoint: s3_endpoint.to_owned(),
    };
    let credentials = Credentials {
        access_key: Some(s3_access_key.to_owned()),
        secret_key: Some(s3_secret_key.to_owned()),
        security_token: None,
        session_token: None,
        expiration: None,
    };

    let bucket = Bucket::new(&s3_bucket, region, credentials).unwrap();

    Config {
        pool: db_pool,
        address: server_address,
        jwt: jwt_secret,
        s3: bucket,
    }
}
