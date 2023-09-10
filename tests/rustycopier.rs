use std::{fs::create_dir, process::Command};

use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
use assert_fs::prelude::*;
#[path = "../utils/test_utils.rs"] //
mod test_utils;
#[test]
fn copy_one_file_into_a_folder() -> Result<(), Box<dyn std::error::Error>> {
    let temp_root = assert_fs::TempDir::new().unwrap();
    let file = temp_root.child("poetry.txt");
    let poetry = test_utils::get_random_poetry();
    file.write_str(poetry).unwrap();
    let temp_folder = temp_root.child("my_targer_folder");
    create_dir(&temp_folder).unwrap();
    let mut cmd = Command::cargo_bin("rustycopier")?;
    cmd.arg("--paths")
        .arg(file.to_str().unwrap())
        .arg(temp_folder.to_str().unwrap());
    cmd.assert().success();
    let copied_file = temp_folder.child("poetry.txt");
    copied_file.assert(poetry);
    Ok(())
}
