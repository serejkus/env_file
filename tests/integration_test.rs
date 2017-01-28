extern crate env_file;
extern crate rand;

use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::ops::Drop;


struct FileCleaner<'a> {
    filename: &'a str,
    created: bool,
}

impl<'a> FileCleaner<'a> {
    pub fn new(filename: &'a str) -> Self {
        FileCleaner {
            filename: filename,
            created: false
        }
    }

    pub fn create_file(&mut self) -> io::Result<fs::File> {
        let retval = fs::File::create(self.filename);
        self.created = retval.is_ok();
        retval
    }
}

impl<'a> Drop for FileCleaner<'a> {
    #[allow(unused_must_use)] // for remove_file, because we can't do anything within stack unwinding
    fn drop(&mut self) {
        if self.created {
            fs::remove_file(self.filename);
        }
    }
}

fn generate_random_name() -> String {
    let mut retval = "ENV_FILE_TEST_".to_string();
    retval.push_str(&rand::random::<u64>().to_string());
    retval

}

#[test]
fn not_existing_file() {
    let env_name = generate_random_name();
    assert!(env_file::read(&env_name).is_err());
}

#[test]
fn existing_file() {
    let env_name = generate_random_name();
    let file_name = generate_random_name();

    let mut file_cleaner = FileCleaner::new(&file_name);

    let file_contents = generate_random_name();
    {
        let mut f = file_cleaner.create_file().unwrap();
        f.write_all(file_contents.as_bytes()).unwrap();
    }

    env::set_var(&env_name, &file_name);

    assert_eq!(file_contents, env_file::read(&env_name).expect("failed to read file"));
}
