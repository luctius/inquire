//! This module contains the type aliases for functions called as validators
//! of a given input.
//!
//! It also provides several built-in validators generated through macros,
//! exported with the `builtin_validators` feature.

use crate::answer::OptionAnswer;

/// Type alias for validators that receive a string slice as the input,
/// such as [Text](crate::Text) and [Password](crate::Password).
/// When creating containers of validators, you might need to type hint
/// them using this type.
///
/// If the input provided by the user is invalid, your validator should return [Ok(())].
///
/// If the input is not valid, your validator should return [Err(String)],
/// where the content of [Err] is a string whose content will be displayed
/// to the user as an error message. It is recommended that this value gives
/// a helpful feedback to the user, e.g. "Your password should contain at least 8 characters".
pub type StringValidator<'a> = &'a dyn Fn(&str) -> Result<(), String>;

/// Type alias for validators used in [DateSelect](crate::DateSelect) prompts.
/// When creating containers of validators, you might need to type hint
/// them using this type.
///
/// If the input provided by the user is invalid, your validator should return [Ok(())].
///
/// If the input is not valid, your validator should return [Err(String)],
/// where the content of [Err] is a string whose content will be displayed
/// to the user as an error message. It is recommended that this value gives
/// a helpful feedback to the user, e.g. "Setting your appointment on Saturdays is not allowed".
#[cfg(feature = "date")]
pub type DateValidator<'a> = &'a dyn Fn(chrono::NaiveDate) -> Result<(), String>;

/// Type alias for validators used in [MultiSelect](crate::MultiSelect) prompts.
/// When creating containers of validators, you might need to type hint
/// them using this type.
///
/// If the input provided by the user is invalid, your validator should return [Ok(())].
///
/// If the input is not valid, your validator should return [Err(String)],
/// where the content of [Err] is a string whose content will be displayed
/// to the user as an error message. It is recommended that this value gives
/// a helpful feedback to the user, e.g. "You should select at most two options".
pub type MultiOptionValidator<'a> = &'a dyn Fn(&[OptionAnswer]) -> Result<(), String>;

/// Built-in validator that checks whether the answer is not empty.
///
/// # Arguments
///
/// * `$message` - optional - Error message returned by the validator.
///   Defaults to "A response is required."
///
/// # Examples
///
/// ```
/// use inquire::{required, validator::StringValidator};
///
/// let validator: StringValidator = required!();
/// assert_eq!(Ok(()), validator("Generic input"));
/// assert_eq!(Err(String::from("A response is required.")), validator(""));
///
/// let validator: StringValidator = required!("No empty!");
/// assert_eq!(Ok(()), validator("Generic input"));
/// assert_eq!(Err(String::from("No empty!")), validator(""));
/// ```
#[macro_export]
#[cfg(feature = "builtin_validators")]
macro_rules! required {
    () => {
        $crate::required! {"A response is required."}
    };

    ($message:expr) => {
        &|a| match a.is_empty() {
            true => Err(String::from($message)),
            false => Ok(()),
        }
    };
}

/// Built-in validator that checks whether the answer length is smaller than
/// or equal to the specified threshold.
///
/// Be careful when using this as a StringValidator. The `len()` method used
/// in this validator is not the best tool for that. See this
/// [StackOverflow question](https://stackoverflow.com/questions/46290655/get-the-string-length-in-characters-in-rust)
///
/// # Arguments
///
/// * `$length` - Maximum length of the input.
/// * `$message` - optional - Error message returned by the validator.
///   Defaults to "The length of the response should be at most $length"
///
/// # Examples
///
/// ```
/// use inquire::{max_length, validator::StringValidator};
///
/// let validator: StringValidator = max_length!(5);
/// assert_eq!(Ok(()), validator("Good"));
/// assert_eq!(Err(String::from("The length of the response should be at most 5")), validator("Terrible"));
///
/// let validator: StringValidator = max_length!(5, "Not too large!");
/// assert_eq!(Ok(()), validator("Good"));
/// assert_eq!(Err(String::from("Not too large!")), validator("Terrible"));
/// ```
#[macro_export]
#[cfg(feature = "builtin_validators")]
macro_rules! max_length {
    ($length:expr) => {
        $crate::max_length! {$length, format!("The length of the response should be at most {}", $length)}
    };

    ($length:expr, $message:expr) => {
        {
            &|a| match a.len() {
                _len if _len <= $length => Ok(()),
                _ => Err(String::from($message)),
            }

        }
    };
}

/// Built-in validator that checks whether the answer length is larger than
/// or equal to the specified threshold.
///
/// Be careful when using this as a StringValidator. The `len()` method used
/// in this validator is not the best tool for that. See this
/// [StackOverflow question](https://stackoverflow.com/questions/46290655/get-the-string-length-in-characters-in-rust)
///
/// # Arguments
///
/// * `$length` - Minimum length of the input.
/// * `$message` - optional - Error message returned by the validator.
///   Defaults to "The length of the response should be at least $length"
///
/// # Examples
///
/// ```
/// use inquire::{min_length, validator::StringValidator};
///
/// let validator: StringValidator = min_length!(3);
/// assert_eq!(Ok(()), validator("Yes"));
/// assert_eq!(Err(String::from("The length of the response should be at least 3")), validator("No"));
///
/// let validator: StringValidator = min_length!(3, "You have to give me more than that!");
/// assert_eq!(Ok(()), validator("Yes"));
/// assert_eq!(Err(String::from("You have to give me more than that!")), validator("No"));
/// ```
#[macro_export]
#[cfg(feature = "builtin_validators")]
macro_rules! min_length {
    ($length:expr) => {
        $crate::min_length! {$length, format!("The length of the response should be at least {}", $length)}
    };

    ($length:expr, $message:expr) => {
        {
            &|a| match a.len() {
                _len if _len >= $length => Ok(()),
                _ => Err(String::from($message)),
            }
        }
    };
}

/// Built-in validator that checks whether the answer length is equal to
/// the specified value.
///
/// Be careful when using this as a StringValidator. The `len()` method used
/// in this validator is not the best tool for that. See this
/// [StackOverflow question](https://stackoverflow.com/questions/46290655/get-the-string-length-in-characters-in-rust)
///
/// # Arguments
///
/// * `$length` - Expected length of the input.
/// * `$message` - optional - Error message returned by the validator.
///   Defaults to "The length of the response should be $length"
///
/// # Examples
///
/// ```
/// use inquire::{length, validator::StringValidator};
///
/// let validator: StringValidator = length!(3);
/// assert_eq!(Ok(()), validator("Yes"));
/// assert_eq!(Err(String::from("The length of the response should be 3")), validator("No"));
///
/// let validator: StringValidator = length!(3, "Three characters please.");
/// assert_eq!(Ok(()), validator("Yes"));
/// assert_eq!(Err(String::from("Three characters please.")), validator("No"));
/// ```
#[macro_export]
#[cfg(feature = "builtin_validators")]
macro_rules! length {
    ($length:expr) => {
        $crate::length! {$length, format!("The length of the response should be {}", $length)}
    };

    ($length:expr, $message:expr) => {{
        &|a| match a.len() {
            _len if _len == $length => Ok(()),
            _ => Err(String::from($message)),
        }
    }};
}

/// Built-in validator that checks whether the answer is able to be successfully
/// parsed to a given type, such as f64.
/// [The given type must implement the FromStr trait.](https://doc.rust-lang.org/stable/std/primitive.str.html#method.parse)
///
/// # Arguments
///
/// * `$type` - Target type of the parsing operation.
/// * `$message` - optional - Error message returned by the validator.
///   Defaults to "Failure when parsing response to type $type"
///
/// # Examples
///
/// ```
/// use inquire::{parse_primitive, validator::StringValidator};
///
/// let validator: StringValidator = parse_primitive!(f64);
/// assert_eq!(Ok(()), validator("32.44"));
/// assert_eq!(Err(String::from("Failure when parsing response to type f64")), validator("32f"));
///
/// let validator: StringValidator = parse_primitive!(f64, "Invalid number");
/// assert_eq!(Ok(()), validator("11e15"));
/// assert_eq!(Err(String::from("Invalid number")), validator("11^2"));
/// ```
#[macro_export]
#[cfg(feature = "builtin_validators")]
macro_rules! parse_primitive {
    ($type:ty) => {
        $crate::parse_primitive! {$type, format!("Failure when parsing response to type {}", std::any::type_name::<$type>())}
    };

    ($type:ty, $message:expr) => {{
        &|a| match a.parse::<$type>() {
            Ok(_) => Ok(()),
            Err(err) => Err(String::from($message)),
        }
    }};
}
