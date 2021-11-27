use batch_run::Batch;
use std::{
    fs::{create_dir, read_to_string, File},
    io::Write,
    path::{Path, PathBuf}, collections::HashMap,
};
use serde::Deserialize;
use toml::from_str;

#[derive(Deserialize, Default)]
struct Data {
    types: String,
    definition: String,
    value: String,
}

impl Data {
    fn write(&self, name: &str, path: impl AsRef<Path>) {
        let mut path = path.as_ref().to_owned();
        path.push(name);
        if !path.exists() {
            create_dir(&path).unwrap();
        }
        path.push("dummy"); // a hack, so that folder isn't overwritten with file name
        write!(
            File::create(&path.with_file_name(format!("{}-main.rs", name))).unwrap(),
            include_str!("main.tpl"),
            name = name,
            value = self.value
        )
        .unwrap();
        write!(
            File::create(&path.with_file_name("definition.rs")).unwrap(),
            include_str!("definition.tpl"),
            definition = self.definition
        )
        .unwrap();
        write!(
            File::create(&path.with_file_name(format!("{}-user.rs", name))).unwrap(),
            include_str!("user.tpl"),
            types = self.types, 
            ser_type = self.types.split(",").next().unwrap(),
            value = self.value
        ).unwrap();
        write!(
            File::create(&path.with_file_name(format!("{}-main.snapshot", name))).unwrap(),
            include_str!("main.snapshot.tpl"),
            name = name
        ).unwrap();
        write!(
            File::create(&path.with_file_name(format!("{}-user.snapshot", name))).unwrap(),
            include_str!("user.snapshot.tpl"),
        ).unwrap();
    }
}

#[test]
fn main() {
    let toml = read_to_string("test_fixtures/data.toml").unwrap();
    let data: HashMap<String, Data> = from_str(&toml).unwrap();
    let path: PathBuf = "test_fixtures".into();
    data.into_iter().for_each(|(key, value)| value.write(&key, &path));

    let b = Batch::new();
    b.run_match("test_fixtures/**/*-main.rs");
    b.run().unwrap().assert_all_ok();
}
