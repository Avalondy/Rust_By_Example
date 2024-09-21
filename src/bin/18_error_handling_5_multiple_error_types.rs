mod defining_an_error_type {
    use std::fmt;

    type Result<T> = std::result::Result<T, DoubleError>;

    // Define our error types. These may be customized for our error handling cases.
    // Now we will be able to write our own errors, defer to an underlying error
    // implementation, or do something in between.
    struct DoubleError;

    // Generation of an error is completely separate from how it is displayed.
    // There's no need to be concerned about cluttering complex logic with the display style.
    //
    // Note that we don't store any extra info about the errors. This means we can't state
    // which string failed to parse without modifying our types to carry that information.
    impl fmt::Display for DoubleError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        vec.first()
            .ok_or(DoubleError)
            .and_then(|s| s.parse::<i32>().map_err(|_| DoubleError).map(|i| i * 2))
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    pub fn test() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }
}

mod boxing_errors {
    use std::{error, fmt};

    // Change the alias to use `Box<dyn error::Error>`.
    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    #[derive(Debug)]
    struct EmptyVec;

    impl fmt::Display for EmptyVec {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    impl error::Error for EmptyVec {}

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        vec.first()
            .ok_or_else(|| EmptyVec.into()) // Converts to Box
            .and_then(|s| s.parse::<i32>().map_err(|e| e.into()).map(|i| i * 2))
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    pub fn test() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }
}

mod other_uses_of_question_mark {
    // `?` actually means `unwrap` or `return Err(From::from(err))`
    // So it's possible to convert from the current error to the return type
    // as long as the `From::from` trait has been implemented
    use std::{error, fmt};

    // Change the alias to use `Box<dyn error::Error>`.
    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    #[derive(Debug)]
    struct EmptyVec;

    impl fmt::Display for EmptyVec {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    impl error::Error for EmptyVec {}

    // The same structure as before but rather than chain all `Results`
    // and `Options` along, we `?` to get the inner value out immediately.
    fn double_first(vec: Vec<&str>) -> Result<i32> {
        let first = vec.first().ok_or(EmptyVec)?;
        let parsed = first.parse::<i32>()?;
        Ok(parsed * 2)
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    pub fn test() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }
}

mod wrapping_errors {
    // An alternative to boxing errors is to wrap them in your own error type.
    use std::{
        error::{self, Error},
        fmt,
        num::ParseIntError,
    };

    type Result<T> = std::result::Result<T, DoubleError>;

    #[derive(Debug)]
    enum DoubleError {
        EmptyVec,
        // We will defer to the parse error implementation for their error.
        // Supplying extra info requires adding more data to the type.
        Parse(ParseIntError),
    }

    impl fmt::Display for DoubleError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),
                // The wrapped error contains additional information and is available
                // via the source() method.
                DoubleError::Parse(..) => {
                    write!(f, "the provided string could not be parsed as int")
                }
            }
        }
    }

    impl error::Error for DoubleError {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            match self {
                DoubleError::EmptyVec => None,
                // The cause is the underlying implementation error type. Is implicitly
                // cast to the trait object `&error::Error`. This works because the
                // underlying type already implements the `Error` trait.
                DoubleError::Parse(e) => Some(e),
            }
        }
    }

    // Implement the conversion from `ParseIntError` to `DoubleError`.
    // This will be automatically called by `?` if a `ParseIntError`
    // needs to be converted into a `DoubleError`.
    impl From<ParseIntError> for DoubleError {
        fn from(value: ParseIntError) -> Self {
            Self::Parse(value)
        }
    }

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        let first = vec.first().ok_or(DoubleError::EmptyVec)?;
        // Here we implicitly use the `ParseIntError` implementation of `From` (which
        // we defined above) in order to create a `DoubleError`.
        let parsed = first.parse::<i32>()?;
        Ok(parsed * 2)
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => {
                println!("Error: {}", e);
                if let Some(source) = e.source() {
                    println!("  Caused by: {}", source);
                }
            }
        }
    }

    pub fn test() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }
}

fn main() {
    defining_an_error_type::test();
    boxing_errors::test();
    other_uses_of_question_mark::test();
    wrapping_errors::test();
}
