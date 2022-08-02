use std::any::Any;

/// A simple struct that contains checking functions
pub struct Checker;

/// Checker struct implementation
impl Checker {
    /// Verify the type of variable with a generic as a type.
    ///
    /// This function takes a reference to any of your type and check if the type into the generic is the same that the type of your data.
    /// ```
    /// use checking::Checker;
    ///
    /// fn main() {
    ///     let result: bool = Checker::type_of::<i32>(&"test".to_string());
    ///
    ///     match result {
    ///         true => println!("Correct type"),
    ///         false => println!("Incorrect type"),
    ///     }
    ///
    ///     // Expected result: Correct type
    /// }
    /// ```
    pub fn type_of<T: Any>(var: &dyn Any) -> bool {
        var.is::<T>()
    }

    /// Executes a logical equal on both datas
    ///
    /// All datas needs to derive PartialEq.
    /// ```
    /// use checking::Checker;
    ///
    /// #[derive(PartialEq)]
    /// struct Test {
    ///     a: u8,
    /// }
    ///
    /// fn main() {
    ///     let a: Test = Test {
    ///         a: 2,
    ///     };
    ///     let b: Test = Test {
    ///         a: 8,
    ///     };
    ///
    ///     let result: bool = Checker::equals_to(a, b);
    ///
    ///     match result {
    ///         true => println!("Equal!"),
    ///         false => println!("Not equal"),
    ///     }
    ///
    ///     // Expected result: Equal!
    /// }
    /// ```
    pub fn equals_to<T: PartialEq>(a: T, b: T) -> bool {
        a == b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_type() {
        let result = Checker::type_of::<String>(&"test".to_string());

        assert_eq!(result, true)
    }
}