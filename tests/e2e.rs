use assert_cmd::prelude::*;
use std::process::Command;
use tempdir::TempDir;

#[test]
fn add_item() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = TempDir::new("_")?;
    let mut cmd = Command::cargo_bin("todo")?;
    let storage_file_path = String::from(tmp_dir.path().to_str().unwrap()) + "todo.json";

    let add_item_assertion = cmd
        .arg("add")
        .arg("item 1")
        .arg("--storage-file")
        .arg(&storage_file_path)
        .assert();

    let list_items_assertion = cmd
        .arg("ls")
        .arg("--storage-file")
        .arg(&storage_file_path)
        .assert();

    add_item_assertion.stdout("OK");
    list_items_assertion.stdout("item 1");
    Ok(())
}
