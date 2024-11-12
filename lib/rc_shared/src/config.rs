#[macro_export] macro_rules! config {
    ($var:expr) => {
        {
            use dotenvy_macro::dotenv;
            match option_env!($var) {
                Some(value) => value,
                None => dotenv!($var)
            }
        }
    };
}
