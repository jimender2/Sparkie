mod config;

fn main() {
    println!("Hello, world!");
    let config_values = config::config();

    println!("{:?}test", config_values);
}
