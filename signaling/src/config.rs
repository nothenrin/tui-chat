use std::{fmt::Debug, str::FromStr};

use redis::SetExpiry;

fn get_var<T>(var_name: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    std::env::var(var_name)
        .expect(&format!("env variable not found: {}", var_name))
        .parse()
        .expect(&format!("failed to parse env variable: {}", var_name))
}

#[derive(Clone)]
pub struct RedisConfig {
    pub url: String,
    pub ttl: SetExpiry,
}

impl RedisConfig {
    pub fn from_env() -> Self {
        Self {
            url: get_var("AWS_REGION"),
            ttl: SetExpiry::EX(5 * 60),
        }
    }
}

#[derive(Clone)]
pub struct Config {
    pub redis: RedisConfig,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().expect("failed to find .env file");
        let redis = RedisConfig::from_env();

        Self { redis }
    }
}
