pub mod easy;
pub mod medium;

pub trait Matchable {
    type Target<'a>
    where
        Self: 'a;

    fn as_match(&self) -> Self::Target<'_>;
}

impl Matchable for String {
    type Target<'a> = &'a str;

    fn as_match(&self) -> &str {
        self.as_str()
    }
}

impl Matchable for &str {
    type Target<'a>
        = &'a str
    where
        Self: 'a;

    fn as_match(&self) -> &str {
        self
    }
}

macro_rules! impl_matchable_copy {
    ($($t:ty),*) => {
        $(
            impl Matchable for $t {
                type Target<'a> = $t;

                fn as_match(&self) -> $t {
                    *self
                }
            }
        )*
    };
}

impl_matchable_copy!(i32, i64, u32, u64, usize, isize, bool, char, f32, f64);

#[macro_export]
macro_rules! check_case {
    ($name:ident, $val:expr, $patron:pat) => {
        #[test]
        fn $name() {
            let res = $val;
            println!("Output: {:?}, Expected: {}", res, stringify!($patron));
            assert!(matches!($crate::Matchable::as_match(&res), $patron));
        }
    };
}
