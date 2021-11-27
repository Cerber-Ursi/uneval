use std::{fs::File, path::PathBuf, io::Write};

fn main() {
    let path: PathBuf = [std::env::var("OUT_DIR").unwrap(), "from_tuple.rs".into()].iter().collect();
    let mut convert = File::create(path).expect("Failed to create file for tuples conversions");
    for i in 1..=32 {
        let array = format!("[T; {}]", i);
        let tuple = format!("({})", (0..i).map(|_| "T,").collect::<String>());
        let mapping = format!("[{}]", (0..i).map(|index| format!("tuple.{}", index)).collect::<Vec<_>>().join(","));
        write!(convert, "
            impl<T> FromTuple<{tuple}> for {array} {{
                #[inline]
                fn from_tuple(tuple: {tuple}) -> Self {{
                    {mapping}
                }}
            }}
        ", array = array, tuple = tuple, mapping = mapping).expect("Failed to write the conversion code");
        let types = (0..i).map(|index| format!("T{}", index)).collect::<Vec<_>>().join(",");
        write!(convert, "
            impl<{types}> FromTuple<({types},)> for ({types},) {{
                #[inline]
                fn from_tuple(tuple: ({types},)) -> Self {{
                    tuple
                }}
            }}
            #[inline]
            pub fn convert_tuple_{count}<{types}, Out: FromTuple<({types},)>>(tuple: ({types},)) -> Out {{
                Out::from_tuple(tuple)
            }}
        ", types = types, count = i).expect("Failed to write the conversion code");
    }
}