use assert_cmd::{assert::Assert, prelude::*};
use std::process::Command;
use tempdir::TempDir;

pub struct App {
    storage_path: String,
}

impl App {
    pub fn from_temp_file() -> App {
        let tmp_dir = TempDir::new("_").unwrap();
        let storage_path = String::from(tmp_dir.path().to_str().unwrap()) + "todo.json";
        App { storage_path }
    }

    pub fn add<T: AsRef<str>>(&self, item: T) -> Assert {
        Command::cargo_bin("todo")
            .unwrap()
            .arg("--storage-path")
            .arg(&self.storage_path)
            .arg("add")
            .arg(item.as_ref())
            .assert()
    }

    pub fn ls(&self) -> Assert {
        Command::cargo_bin("todo")
            .unwrap()
            .arg("--storage-path")
            .arg(&self.storage_path)
            .arg("ls")
            .assert()
    }

    pub fn rm<T: AsRef<str>>(&self, item: T) -> Assert {
        Command::cargo_bin("todo")
            .unwrap()
            .arg("--storage-path")
            .arg(&self.storage_path)
            .arg("rm")
            .arg(item.as_ref())
            .assert()
    }
}
