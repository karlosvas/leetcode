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

/// Trait implemented by per-problem `ListNode` types so they can be
/// stringified for test assertions via [`list_to_string`].
pub trait ListLike {
    fn val(&self) -> i32;
    fn next(&self) -> &Option<Box<Self>>;
}

/// Implements [`ListLike`] for a `ListNode` type with `val: i32` and
/// `next: Option<Box<Self>>` fields.
#[macro_export]
macro_rules! impl_list_like {
    ($t:ty) => {
        impl $crate::ListLike for $t {
            fn val(&self) -> i32 {
                self.val
            }

            fn next(&self) -> &Option<Box<Self>> {
                &self.next
            }
        }
    };
}

/// Renders a singly linked list as `"[1,2,3]"` for use with [`check_case!`].
pub fn list_to_string<T: ListLike>(head: &Option<Box<T>>) -> String {
    let mut vals = Vec::new();
    let mut current = head.as_ref();
    while let Some(node) = current {
        vals.push(node.val().to_string());
        current = node.next().as_ref();
    }
    format!("[{}]", vals.join(","))
}

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
