/// Create [rustc_hash::FxHashMap]'s using a readable syntax, similar to dicts in python or objects in js. Adapted from maplit to support `FxHashMap`
///
/// ## Example
///
/// ```
/// let map = hashmap!{
///     "a" => 1,
///     "b" => 2,
/// };
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
/// ```
#[macro_export]
macro_rules! hashmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { $crate::hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = $crate::hashmap!(@count $($key),*);
            let mut _map = rustc_hash::FxHashMap::with_capacity_and_hasher(_cap, Default::default());
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}

/// Create [rustc_hash::FxHashSet]'s using a readable syntax. Adapted from maplit to support `FxHashSet`
///
/// ## Example
///
/// ```
/// let set = hashset!{"a", "b"};
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(!set.contains("c"));
/// ```
#[macro_export]
macro_rules! hashset {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashset!(@single $rest)),*]));

    ($($key:expr,)+) => { $crate::hashset!($($key),+) };
    ($($key:expr),*) => {
        {
            let _cap = $crate::hashset!(@count $($key),*);
            let mut _set = rustc_hash::FxHashSet::with_capacity_and_hasher(_cap, Default::default());
            $(
                let _ = _set.insert($key);
            )*
            _set
        }
    };
}
