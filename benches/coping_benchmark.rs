use std::fs::create_dir;
#[cfg(test)]
use std::path::Path;

use assert_fs::prelude::{FileWriteStr, PathAssert, PathChild};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
#[path = "../src/copier/mod.rs"] //
mod copier;
#[path = "../src/copier_pool/mod.rs"] //
mod copier_pool;
#[path = "../src/progress_counter/mod.rs"] //
mod progress_counter;
#[path = "../src/utils/mod.rs"] //
mod utils;
fn copy_to_path(source: &Path, target_path: &Path) {
    let source_path = &source.to_path_buf();
    let mut folder = copier::FileToCopy::from_path(source_path);
    folder.load_files_from_path().unwrap();
    folder.copy_to(target_path, false);
}
fn copy_200_files(c: &mut Criterion) {
    const NUMBER_OF_FILES: usize = 200;
    c.bench_function("coping_benchmark", |b| {
        b.iter(|| copy_files(black_box(NUMBER_OF_FILES)));
    });
}
fn copy_files(number_of_files: usize) {
    let temp_root = assert_fs::TempDir::new().unwrap();
    let poetry = utils::test_utils::get_random_poetry();
    let source_folder = utils::test_utils::create_temp_child_folder(&temp_root, "my_source_folder");
    for file_number in 0..number_of_files {
        let _ = utils::test_utils::create_temp_child_file(
            &temp_root,
            format!("my_source_folder/poetry{}.txt", file_number).as_str(),
            poetry,
        );
    }
    let target_folder = utils::test_utils::create_temp_child_folder(&temp_root, "my_targer_folder");
    copy_to_path(&source_folder, &target_folder);
    (0..number_of_files)
        .into_par_iter()
        .for_each(|file_number| {
            let copied_file =
                temp_root.child(format!("my_source_folder/poetry{}.txt", file_number));
            copied_file.assert(poetry);
        });
}

criterion_group!(benches, copy_200_files);
criterion_main!(benches);
