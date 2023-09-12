use std::fs::create_dir;

use assert_fs::{
    fixture::ChildPath,
    prelude::{FileWriteStr, PathChild},
    TempDir,
};
#[allow(dead_code)]
pub fn get_random_poetry<'a>() -> &'a str {
    "In the world of coding, a language stands
                            Rust, they call it, with its own demands
                            It's strict and crotchety, some may say
                            But its power and speed are here to stay

                            Its syntax may seem a bit obtuse
                            But its memory safety is no excuse
                            For sloppy code that could bring down
                            An entire system with just one frown

                            Rust is compiled, not interpreted
                            And its performance can't be debated
                            It's perfect for systems and low-level tasks
                            And its community is growing fast

                            So if you're feeling crotchety today
                            Give Rust a chance, don't turn away
                            It may be strict, but it's worth the fight
                            For a language that's powerful and right."
}
#[allow(dead_code)]
pub fn create_temp_child_folder(root: &TempDir, folder_name: &str) -> ChildPath {
    let target_folder = root.child(folder_name);
    create_dir(&target_folder).unwrap();
    target_folder
}
#[allow(dead_code)]
pub fn create_temp_child_file(root: &TempDir, file_path: &str, data: &str) -> ChildPath {
    let file = root.child(file_path);
    file.write_str(data).unwrap();
    file
}
