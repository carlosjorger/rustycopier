pub struct Config {
    pub source_path: String,
    pub target_path: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let source_path = args[1].clone();
        let target_path = args[2].clone();

        Ok(Self {
            source_path,
            target_path,
        })
    }
}
