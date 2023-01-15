use assert_cmd::prelude::*;
use std::process::Command;
use tempdir::TempDir;

#[test]
fn add_item() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = TempDir::new("_")?;
    let storage_file_path = String::from(tmp_dir.path().to_str().unwrap()) + "todo.json";

    let add_item_assertion = Command::cargo_bin("todo")?
        .arg("--storage-path")
        .arg(&storage_file_path)
        .arg("add")
        .arg("item 1")
        .assert();

    let list_items_assertion = Command::cargo_bin("todo")?
        .arg("--storage-path")
        .arg(&storage_file_path)
        .arg("ls")
        .assert();

    add_item_assertion.stdout("OK\n");
    list_items_assertion.stdout("item 1\n");
    Ok(())
}
