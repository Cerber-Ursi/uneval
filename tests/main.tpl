use corona::to_file;
use batch_run::{{Batch, config::Config}};

mod definition;

fn main() {{
    to_file({value}, "test_fixtures/{name}/generated.rs").unwrap();
    let b = Batch::new();
    b.run_match("test_fixtures/{name}/{name}-user.rs");
    b.run_with_config(Config::from_env().unwrap().with_stderr_no_color()).unwrap().assert_all_ok();
}}