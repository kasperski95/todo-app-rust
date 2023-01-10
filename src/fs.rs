use std::path::Path;

pub trait Fs {
    fn read_to_string(&self, file_path: &Path) -> String;
}

struct FakeFs {}
impl Fs for FakeFs {
    fn read_to_string(&self, _file_path: &Path) -> String {
        "foo".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let fs = FakeFs {};
        fs.read_to_string(Path::new("foo.txt"));
    }
}
