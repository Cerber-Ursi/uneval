pub trait FromTuple<T> {
    fn from_tuple(tuple: T) -> Self;
}

include!(concat!(env!("OUT_DIR"), "/from_tuple.rs"));