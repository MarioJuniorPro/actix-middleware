use serde::Deserialize;
fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8080
}


  #[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_host" )]
    pub host: String,
    #[serde(default = "default_port" )]
    pub port: u16,
    // #[serde(default="default_port")]
    // bar: Option<String>,
}

impl Config {
    // pub fn new() -> Result<Self, std::env::VarError> {
    //     let config: Config = envy::from_env()?;
    //     Ok(config)
    // }

    // pub fn get_bind_addr(&self) -> std::net::SocketAddr {
    //     format!("{}:{}", self.host, self.port).parse().unwrap()
    // }

    pub fn get_bind_addr(&self) -> std::net::SocketAddr {
        format!("{}:{}", self.host, self.port).parse().unwrap()
    }
}