use std::process::Command;

use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
use assert_fs::prelude::*;
#[path = "../src/utils/mod.rs"] //
mod utils;
#[test]
fn copy_one_file_into_a_folder() -> Result<(), Box<dyn std::error::Error>> {
    let temp_root = assert_fs::TempDir::new().unwrap();
    let poetry = "In the world of coding, a language stands
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
    For a language that's powerful and right.";
    let file = utils::test_utils::create_temp_child_file(&temp_root, "poetry.txt", poetry);
    let temp_folder = utils::test_utils::create_temp_child_folder(&temp_root, "my_targer_folder");
    let mut cmd = Command::cargo_bin("rustycopier")?;
    cmd.arg("--paths")
        .arg(file.to_str().unwrap())
        .arg(temp_folder.to_str().unwrap())
        .arg("-d");
    cmd.assert().success();
    let copied_file = temp_folder.child("poetry.txt");
    copied_file.assert(poetry);
    Ok(())
}
