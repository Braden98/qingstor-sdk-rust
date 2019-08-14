use std::error::Error;
use super::super::config::*;

#[derive(Debug)]
pub struct Service<'a> {
	config: &'a Config,
}

impl<'a> Service<'a> {

	pub fn init(c: &mut Config) -> Result<Service, Box<dyn Error>> {
        c.check()?;
        Ok(Service{
            config: c,
        })
	}

        pub fn bucket(c: &mut Config,bucket_name:String,Zone:String) ->Result<Bucket,Box<dyn Error>>{
            c.check()?;
            Ok(Bucket{
                config:c,
                bucket_name,
                zone:Zone,
            })

        }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn init_service() {
        let s = "
access_key_id: access_key
secret_access_key: secret
protocol: https
";

        let mut c:Config = Config::load_from_str(&s).unwrap();
        Service::init(&mut c).unwrap();
    }
}
