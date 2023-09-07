use clap::CommandFactory;

#[path = "src/config.rs"]
mod config;

fn main() -> std::io::Result<()> {
    let outdir = match std::env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };
    let out_dir = std::path::PathBuf::from(outdir);
    let cmd = config::Config::command();

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(out_dir.join("mybin.1"), buffer)?;

    Ok(())
}
