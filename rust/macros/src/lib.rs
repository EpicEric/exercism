#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };
    ($( $k:expr => $v:expr ),+ $(,)?) => {{
        let mut hashmap = ::std::collections::HashMap::new();
        $(
            hashmap.insert($k, $v);
        )+
        hashmap
    }};
}
