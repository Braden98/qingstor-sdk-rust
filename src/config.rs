use std::prelude::v1::*;
use std::error::Error;
use std::fs::File;
use std::fmt;


#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    HTTP,
    HTTPS,
}

#[derive(Debug)]
pub struct ConfigError {
    msg: String,
}

impl ConfigError {

    fn missing(field: &str) -> ConfigError {
        let msg: String = format!("field \"{}\" cannot be empty", field);
        ConfigError{msg: msg}
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Config error: {}", self.msg)
    }
}

impl Error for ConfigError {
}

#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Config {
    pub access_key_id: String,
    pub secret_access_key: String,

    pub host: String,
    pub port: u16,

    pub connection_retries: i32,
    pub additional_user_agent: String,
    pub log_level: String,

    pub protocol: Protocol,
}

impl Default for Config {
    fn default() -> Config {
        Config{
            access_key_id: String::from(""),
            secret_access_key: String::from(""),

            host: "qingstor.com".to_owned(),
            port: 0,
            protocol: Protocol::HTTPS,
            connection_retries: 3,
            additional_user_agent: "".to_owned(),

            log_level: "INFO".to_owned(),
        }
    }
}

impl Config {

    pub fn new(access_key_id: &String, secret_access_key: &String) -> Config {

        let mut c:Config = Self::default();
        c.access_key_id = access_key_id.clone();
        c.secret_access_key = secret_access_key.clone();
        c
    }


    pub fn load_from_file(path: &str) -> Result<Config, Box<dyn Error>> {

        let f = File::open(path)?;
        let c:Config = serde_yaml::from_reader(f)?;
        Ok(c)
    }

    pub fn load_from_str(content: &str) -> Result<Config, Box<dyn Error>> {

        let c:Config = serde_yaml::from_str(content)?;
        Ok(c)
    }

    pub fn check(&mut self) -> Result<(), ConfigError> {
        if self.access_key_id.len() == 0 {
            return Err(ConfigError::missing("access_key_id"))
        }
        if self.secret_access_key.len() == 0 {
            return Err(ConfigError::missing("secret_access_key"))
        }
        if self.host.len() == 0 {
            return Err(ConfigError::missing("host"))
        }
        self.port = match self.protocol {
            Protocol::HTTP =>  80,
            Protocol::HTTPS => 443,
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::prelude::*;

    #[test]
    fn config_from_str() {
        let s = "
access_key_id: access_key
secret_access_key: secret
protocol: https
";

        let mut c:Config = Config::load_from_str(&s).unwrap();
        assert_eq!(c.access_key_id, "access_key");
        assert_eq!(c.secret_access_key, "secret");
        assert_eq!(c.protocol, Protocol::HTTPS);
        c.check().unwrap();
        println!("config {:?}", c);

        let mut c:Config = Config::default();
        match c.check() {
            Ok(_) => assert!(false, "expect error"),
            Err(err) => println!("{}", err),
        }
    }

    #[test]
    fn config_from_file() {
        let s = "
access_key_id: access_key
secret_access_key: secret
protocol: https
";
        let tmp_path = "/tmp/test_qingstor_sdk.config";
        {
            let mut f = File::create(tmp_path).unwrap();
            f.write_all(s.as_bytes()).unwrap();
        }

        let mut c:Config = Config::load_from_file(tmp_path).unwrap();
        c.check().unwrap();
        assert_eq!(c.access_key_id, "access_key");
        assert_eq!(c.secret_access_key, "secret");
        assert_eq!(c.protocol, Protocol::HTTPS);
    }

     #[test]
    fn config_from_file_missing() {
        let tmp_path = "/tmp/test_qingstor_sdk.non";
        match Config::load_from_file(tmp_path) {
            Ok(_) => assert!(false, "expect error"),
            Err(err) => println!("{:?}", err),
        }
    }
}
