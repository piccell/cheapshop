use dotenv::dotenv;
use rocket::config::{Config, Environment};
use std::env;

pub fn from_env() -> Config {
    dotenv().ok();

    let port = env::var("PORT")
        .expect("PORT non défini")
        .parse::<u16>()
        .expect("PORT invalide");

    let environment = Environment::active().expect("No environment found");

    Config::build(environment)
        .environment(environment)
        .port(port)
        .finalize()
        .unwrap()
}
