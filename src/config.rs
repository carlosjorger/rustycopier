pub struct Config {
    pub source_paths: Vec<String>,
    pub target_path: String,
}
impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let mut rev_args = args.collect::<Vec<String>>().into_iter().rev();
        let target_path = match rev_args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a target_path string"),
        };
        let source_paths: Vec<String> = rev_args.collect();

        Ok(Self {
            source_paths,
            target_path,
        })
    }
}
