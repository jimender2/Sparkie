use dotenv::dotenv;
use std::collections::HashMap;

pub fn config() -> HashMap<&'static str, String> {
    dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set.");

    let config_values = HashMap::from([("DISCORD_TOKEN", token)]);

    return config_values;
}
