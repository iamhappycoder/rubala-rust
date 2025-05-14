pub mod framework;

#[macro_export]
macro_rules! hashmap_s {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($key.to_string(), $value.to_string());
            )*
            map
        }
    };
}

#[cfg(test)]
pub mod test_utils {
    pub use crate::hashmap_s;
}