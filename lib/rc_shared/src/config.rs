#[macro_export] macro_rules! config {
    ($var:expr) => {
        {
            env!($var)
        }
    };
}
