extern crate custom_error;
use custom_error::custom_error;

custom_error! {pub ValidateError
    InvalidLength = "Invalid length detected.",
    NoSpecialChars = "No special chars present.",
    NoUpperCase = "No uppercase chars present.",
    NoLowerCase = "No lowercase chars present.",
    NoNumbers = "No number chars present.",
}
