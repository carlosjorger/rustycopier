pub struct Config {
    pub source_path: String,
    pub target_path: String,
}
impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let source_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a source_path string"),
        };
        let target_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a target_path string"),
        };
        Ok(Self {
            source_path,
            target_path,
        })
    }
}
