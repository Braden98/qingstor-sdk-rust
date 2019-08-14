use::std::fmt;

pub struct Bucket<'a> {
	config: &'a Config,
    bucket_name: String,
    object_key: String,
    zone: String,
}