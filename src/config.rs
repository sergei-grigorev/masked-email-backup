pub mod fake;

pub struct Config {
    pub user_name: String,
    pub storage: String,
}

pub trait ConfigReader {
    fn load() -> Result<Config, std::io::Error>;
}
