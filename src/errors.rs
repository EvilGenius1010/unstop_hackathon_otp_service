use core::fmt;


pub struct DatabaseConnError;

impl fmt::Display for DatabaseConnError{ //error message shown to end user.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nError connecting to db, pls try again later!")
    }
}

impl fmt::Debug for DatabaseConnError { // Debug information for developers
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // // Write the debug message to stderr
        // let mut stderr = io::stderr();
        // writeln!(stderr, "Debug: Invalid connection string or trouble connecting to DB.") 
        //     .expect("Failed to write to stderr");

        // Still implement the trait behavior by writing to the provided `Formatter`
        write!(f, "DatabaseConnError {{ Invalid connection string or trouble connecting to DB. }}")
    }
}


pub struct 