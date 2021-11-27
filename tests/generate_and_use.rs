use batch_run::Batch;
use std::{
    fs::{create_dir, File},
    io::BufRead,
    io::Write,
    path::PathBuf,
};

#[derive(Default)]
struct Data {
    name: String,
    types: String,
    definition: String,
    value: String,
}

impl Data {
    fn write(&mut self, path: &mut PathBuf) {
        path.push(&self.name);
        if !path.exists() {
            create_dir(&path).unwrap();
        }
        path.push("dummy"); // a hack, so that folder isn't overwritten with file name
        write!(
            File::create(&path.with_file_name(format!("{}-main.rs", self.name))).unwrap(),
            include_str!("main.tpl"),
            name = self.name,
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
            File::create(&path.with_file_name(format!("{}-user.rs", self.name))).unwrap(),
            include_str!("user.tpl"),
            types = self.types, 
            ser_type = self.types.split(",").next().unwrap(),
            value = self.value
        ).unwrap();
        write!(
            File::create(&path.with_file_name(format!("{}-main.snapshot", self.name))).unwrap(),
            include_str!("main.snapshot.tpl"),
            name = self.name
        ).unwrap();
        write!(
            File::create(&path.with_file_name(format!("{}-user.snapshot", self.name))).unwrap(),
            include_str!("user.snapshot.tpl"),
        ).unwrap();
        path.pop();
        path.pop();

        self.name.clear();
        self.types.clear();
        self.definition.clear();
        self.value.clear();
    }
}

#[derive(PartialEq, Eq)]
enum State {
    Name,
    Types,
    Definition,
    Value,
}

impl State {
    fn forward(&mut self) {
        use State::*;
        *self = match self {
            Name => Self::Types,
            Types => Self::Definition,
            Definition => Self::Value,
            Value => Self::Name,
        }
    }
}

#[test]
fn main() {
    let data = File::open("test_fixtures/source.data").unwrap();
    let mut out = Data::default();
    let mut state: State = State::Name;
    let mut path: PathBuf = ["test_fixtures"].iter().collect();
    for line in std::io::BufReader::new(data).lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            state.forward();
            if state == State::Name {
                out.write(&mut path);
            }
        } else {
            match state {
                State::Name => out.name = line.trim().to_string(),
                State::Types => out.types = line.trim().to_string(),
                State::Definition => out.definition += &(line + "\n"),
                State::Value => out.value += &(line + "\n"),
            }
        }
    }
    // flush anything already here
    out.write(&mut path);

    let b = Batch::new();
    b.run_match("test_fixtures/**/*-main.rs");
    b.run().unwrap().assert_all_ok();
}
