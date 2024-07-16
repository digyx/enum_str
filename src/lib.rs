/// Macro used to generate an enum with a `FromStr` trait implementation and `as_str` method.
///
/// The enum is identified by the `name` passed to the macro. Enum values are identified by
/// the `key` passed in each tuple. The 'value' is used as the string representation for
/// FromStr and AsStr traits.
///
/// # Example
/// ```
/// use std::str::FromStr;
/// use enum_str::enum_str;
///
/// enum_str! {
///     Fruit,
///     (Apple, "🍎"),
///     (Pineapple, "🍍"),
///     (Strawberry, "🍓"),
/// }
///
/// assert_eq!("🍎", Fruit::Apple.as_str());
/// assert_eq!("🍎", Fruit::Apple.to_string().as_str());
/// assert_eq!(Fruit::Apple, Fruit::from_str("🍎").unwrap());
/// assert_eq!(Fruit::Apple, "🍎".parse().unwrap());
/// ```
#[macro_export]
macro_rules! enum_str {
    ($name:ident, $(($key:ident, $value:expr),)*) => {
        #[derive(Debug, PartialEq)]
       enum $name
        {
            $($key),*
        }

        impl $name {
            fn as_str(&self) -> &str {
                match self {
                    $(
                        &$name::$key => $value
                    ),*
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        &$name::$key => write!(f, "{}", $value)
                    ),*
                }
            }
        }

        impl std::str::FromStr for $name {
            type Err = ();

            fn from_str(val: &str) -> Result<Self, Self::Err> {
                match val {
                    $(
                        $value => Ok($name::$key)
                    ),*,
                    _ => Err(())
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::enum_str;
    use std::str::FromStr;

    enum_str! {
        Fruit,
        (Apple, "🍎"),
        (Pineapple, "🍍"),
        (Strawberry, "🍓"),
    }

    #[test]
    fn test_as_str() {
        assert_eq!("🍎", Fruit::Apple.as_str());
    }

    #[test]
    fn test_to_string() {
        assert_eq!("🍎", Fruit::Apple.to_string().as_str());
    }

    #[test]
    fn test_from_str_ok() {
        assert_eq!(Fruit::Apple, Fruit::from_str("🍎").unwrap());
        assert_eq!(Fruit::Strawberry, Fruit::from_str("🍓").unwrap());
    }

    #[test]
    fn test_from_str_err() {
        assert!(Fruit::from_str("Strawberry").is_err());
    }
}
