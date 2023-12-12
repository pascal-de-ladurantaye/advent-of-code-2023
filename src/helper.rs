use std::fmt::Debug;

/// Helper function that prints a debug representation of the given item and returns it.
/// Useful for debugging in the middle of a chain of function calls within an iterator.
///
/// # Example
/// ```
/// use advent_of_code::debug_item;
///
/// let v = vec![1, 2, 3, 4, 5];
/// let sum = v.iter()
///    .map(|x| x * 2)
///    .map(debug_item)
///    .sum::<i32>();
///```
pub fn debug_item<T>(item: T) -> T
where
    T: Debug,
{
    println!("{:#?}", item);
    item
}
