
//! [`break`]: https://doc.rust-lang.org/std/keyword.break.html
//! [`continue`]: https://doc.rust-lang.org/std/keyword.continue.html
//! [`return`]: https://doc.rust-lang.org/std/keyword.return.html
//!
//! Declarative macros for common control-flow use cases such as [`break`], [`continue`], and [`return`].
//!
//! ---
//!
//! [`break_if!(...)`](crate::break_if)
//!
//! [`break`] from a loop if a given predicate evaluates to [`true`].
//!
//! Supports optionally providing a loop label to specify the loop from which to [`break`].
//! ```text
//! use flow_control::break_if;
//!
//! break_if!(predicate);
//! break_if!(predicate, label);
//! ```
//!
//! ---
//!
//! [`continue_if!(...)`](crate::continue_if)
//!
//! [`continue`] to the next iteration of a loop if a given predicate evaluates to [`true`].
//!
//! Supports optionally providing a loop label to specify the loop in which to [`continue`].
//! ```text
//! use flow_control::continue_if;
//!
//! continue_if!(predicate);
//! continue_if!(predicate, label);
//! ```
//!
//! ---
//!
//! [`return_if!(...)`](crate::return_if)
//!
//! [`return`] from a function if a given predicate evaluates to [`true`].
//!
//! Supports optionally providing a value to [`return`].
//! ```text
//! use flow_control::return_if;
//!
//! return_if!(predicate);
//! return_if!(predicate, value);
//! ```
//!
//! ---
//!

/// [`break`]: https://doc.rust-lang.org/std/keyword.break.html
///
/// [`break`] from a loop if a given predicate evaluates to [`true`].
///
/// Supports optionally providing a loop label to specify the loop from which to [`break`].
///
/// # Usage
///
/// [`break_if!`](crate::break_if)`(predicate)`
///
/// [`break_if!`](crate::break_if)`(predicate, label)`
///
/// # Examples
///
/// #### Predicate only
/// ```
/// use flow_control::break_if;
///
/// let mut v = Vec::new();
/// for outer_n in 1..3 {
///     for inner_n in 1..5 {
///         break_if!(inner_n == 3);
///         v.push((outer_n, inner_n));
///     }
/// }
///
/// assert_eq!(
///     v,
///     vec![
///         (1, 1), (1, 2),
///         (2, 1), (2, 2),
///     ]
/// );
/// ```
///
/// #### Predicate and label
/// ```
/// use flow_control::break_if;
///
/// let mut v = Vec::new();
/// 'outer: for outer_n in 1..3 {
///     for inner_n in 1..5 {
///         break_if!(inner_n == 3, 'outer);
///         v.push((outer_n, inner_n));
///     }
/// }
///
/// assert_eq!(
///     v,
///     vec![(1, 1), (1, 2)],
/// );
/// ```
#[macro_export]
macro_rules! break_if {
    ($predicate:expr $(,)?) => {
        if $predicate {
            break;
        }
    };
    ($predicate:expr, $label:tt $(,)?) => {
        if $predicate {
            break $label;
        }
    };
}

/// [`continue`]: https://doc.rust-lang.org/std/keyword.continue.html
///
/// [`continue`] to the next iteration of a loop if a given predicate evaluates to [`true`].
///
/// Supports optionally providing a loop label to specify the loop in which to [`continue`].
///
/// # Usage
///
/// [`continue_if!`](crate::continue_if)`(predicate)`
///
/// [`continue_if!`](crate::continue_if)`(predicate, label)`
///
/// # Examples
///
/// #### Predicate only
/// ```
/// use flow_control::continue_if;
///
/// let mut v = Vec::new();
/// for outer_n in 1..3 {
///     for inner_n in 1..5 {
///         continue_if!(inner_n == 3);
///         v.push((outer_n, inner_n));
///     }
/// }
///
/// assert_eq!(
///     v,
///     vec![
///         (1, 1), (1, 2), (1, 4),
///         (2, 1), (2, 2), (2, 4),
///     ]
/// );
/// ```
///
/// #### Predicate and label
/// ```
/// use flow_control::continue_if;
///
/// let mut v = Vec::new();
/// 'outer: for outer_n in 1..3 {
///     for inner_n in 1..5 {
///         continue_if!(inner_n == 3, 'outer);
///         v.push((outer_n, inner_n));
///     }
/// }
///
/// assert_eq!(
///     v,
///     vec![
///         (1, 1), (1, 2),
///         (2, 1), (2, 2),
///     ]
/// );
/// ```
#[macro_export]
macro_rules! continue_if {
    ($predicate:expr $(,)?) => {
        if $predicate {
            continue;
        }
    };
    ($predicate:expr, $label:tt $(,)?) => {
        if $predicate {
            continue $label;
        }
    };
}

/// [`return`]: https://doc.rust-lang.org/std/keyword.return.html
///
/// [`return`] from a function if a given predicate evaluates to [`true`].
///
/// Supports optionally providing a value to [`return`].
///
/// # Usage
///
/// [`return_if!`](crate::return_if)`(predicate)`
///
/// [`return_if!`](crate::return_if)`(predicate, value)`
///
/// # Examples
///
/// #### Default return
/// ```
/// use flow_control::return_if;
///
/// let mut v = Vec::new();
/// (|| {
///     for n in 1..10 {
///         return_if!(n == 5);
///         v.push(n)
///     }
/// })();
///
/// assert_eq!(v, vec![1, 2, 3, 4]);
/// ```
///
/// #### Return a specified value
/// ```
/// use flow_control::return_if;
///
/// let get_value = || {
///     for n in 1..10 {
///         return_if!(n == 5, "early return");
///     }
///     return "return after loop";
/// };
///
/// assert_eq!(get_value(), "early return");
/// ```
#[macro_export]
macro_rules! return_if {
    ($predicate:expr $(,)?) => {
        if $predicate {
            return;
        }
    };
    ($predicate:expr, $ret:expr $(,)?) => {
        if $predicate {
            return $ret;
        }
    };
}
