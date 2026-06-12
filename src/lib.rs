pub mod easy;
pub mod medium;

#[macro_export]
macro_rules! check_case {
    ($name:ident, $val:expr, $patron:pat) => {
        #[test]
        fn $name() {
            let res = $val;
            println!("Output: {}, Expected: {}", res, stringify!($patron));
            assert!(matches!(res.as_str(), $patron));
        }
    };
}
