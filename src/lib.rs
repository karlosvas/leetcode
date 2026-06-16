pub mod easy;
pub mod medium;

use std::cell::RefCell;
use std::rc::Rc;

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

impl<T> Matchable for Vec<T> {
    type Target<'a>
        = &'a [T]
    where
        T: 'a;

    fn as_match(&self) -> &[T] {
        self.as_slice()
    }
}

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

/// Collects the values of a singly linked list into a `Vec<i32>` for use with [`check_case!`].
pub fn list_to_vec<T: ListLike>(head: &Option<Box<T>>) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut current = head.as_ref();
    while let Some(node) = current {
        vec.push(node.val());
        current = node.next().as_ref();
    }
    vec
}

/// Trait implemented by per-problem `ListNode` types so they can be constructed
/// from a value and a next pointer via [`create_list`].
pub trait ListNew: Sized {
    fn list_new(val: i32, next: Option<Box<Self>>) -> Self;
}

/// Implements [`ListNew`] for a `ListNode` type with `val: i32` and
/// `next: Option<Box<Self>>` fields.
#[macro_export]
macro_rules! impl_list_new {
    ($t:ty) => {
        impl $crate::ListNew for $t {
            fn list_new(val: i32, next: Option<Box<Self>>) -> Self {
                Self { val, next }
            }
        }
    };
}

/// Builds a singly linked list from a slice of values.
pub fn create_list<T: ListNew>(nums: &[i32]) -> Option<Box<T>> {
    let mut head = None;
    for &num in nums.iter().rev() {
        head = Some(Box::new(T::list_new(num, head)));
    }
    head
}

pub trait TreeNodeNew: Sized {
    fn tree_new(
        val: i32,
        left: Option<Rc<RefCell<Self>>>,
        right: Option<Rc<RefCell<Self>>>,
    ) -> Option<Rc<RefCell<Self>>>;
}

#[macro_export]
macro_rules! impl_tree_node_new {
    ($t:ty) => {
        impl $crate::TreeNodeNew for $t {
            fn tree_new(
                val: i32,
                left: Option<std::rc::Rc<std::cell::RefCell<Self>>>,
                right: Option<std::rc::Rc<std::cell::RefCell<Self>>>,
            ) -> Option<std::rc::Rc<std::cell::RefCell<Self>>> {
                Some(std::rc::Rc::new(std::cell::RefCell::new(Self { val, left, right })))
            }
        }
    };
}

pub fn tree_node<T: TreeNodeNew>(
    val: i32,
    left: Option<Rc<RefCell<T>>>,
    right: Option<Rc<RefCell<T>>>,
) -> Option<Rc<RefCell<T>>> {
    T::tree_new(val, left, right)
}

pub fn tree_leaf<T: TreeNodeNew>(val: i32) -> Option<Rc<RefCell<T>>> {
    T::tree_new(val, None, None)
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
