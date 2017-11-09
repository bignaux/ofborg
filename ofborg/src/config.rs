use serde_json;
use std::fs::File;
use std::path::Path;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub checkout: CheckoutConfig,
    pub nix: NixConfig,
    pub rabbitmq: RabbitMQConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RabbitMQConfig {
    pub host: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NixConfig {
    pub system: String,
    pub remote: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckoutConfig {
    pub root: String,
}


impl RabbitMQConfig {
    pub fn as_uri(&self) -> String{
        return format!("amqps://{}:{}@{}//", self.username, self.password, self.host);
    }
}

pub fn load(filename: &Path) -> Config {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let deserialized: Config = serde_json::from_str(&contents).unwrap();

    return deserialized;
}