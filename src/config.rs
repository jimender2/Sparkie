use dotenv::dotenv;
use std::collections::HashMap;

pub fn config() -> HashMap<&'static str, String> {
    dotenv().ok();

    let token = std::env::var("test_token").expect("test_token must be set.");

    let config_values = HashMap::from([("test_token", token),]);

    return config_values;
}
