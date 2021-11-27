use crate::ser::SerResult;
use std::io::Write;

pub(crate) fn tuple_converter(output: impl Write, len: usize) -> SerResult {
    if len > 0 {
        non_zero_size(output, len)
    } else {
        zero_size(output)
    }
}

fn zero_size(mut output: impl Write) -> SerResult {
    write!(output, "
        #[inline]
        fn convert<T>(_: ()) -> [T; 0] {{
            []
        }}
    ")?;
    Ok(())
}

fn non_zero_size(mut output: impl Write, len: usize) -> SerResult {
    write!(
        output,
        "
        trait FromTuple<T>: Sized {{
            fn from_tuple(tuple: T) -> Self;
        }}
    "
    )?;
    let array = format!("[T; {}]", len);
    let tuple = format!("({})", (0..len).map(|_| "T,").collect::<String>());
    let mapping = format!(
        "[{}]",
        (0..len)
            .map(|index| format!("tuple.{}", index))
            .collect::<Vec<_>>()
            .join(",")
    );
    write!(
        output,
        "
            impl<T> FromTuple<{tuple}> for {array} {{
                #[inline]
                fn from_tuple(tuple: {tuple}) -> Self {{
                    {mapping}
                }}
            }}
        ",
        array = array,
        tuple = tuple,
        mapping = mapping
    )?;
    let types = (0..len)
        .map(|index| format!("T{}", index))
        .collect::<Vec<_>>()
        .join(",");
    write!(
        output,
        "
            impl<{types}> FromTuple<({types},)> for ({types},) {{
                #[inline]
                fn from_tuple(tuple: ({types},)) -> Self {{
                    tuple
                }}
            }}

            #[inline]
            fn convert<{types}, Out: FromTuple<({types},)>>(tuple: ({types},)) -> Out {{
                Out::from_tuple(tuple)
            }}
        ",
        types = types
    )?;
    Ok(())
}
