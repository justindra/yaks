// Output port trait - abstraction for displaying results to user

pub trait OutputPort {
    /// Display success message
    fn success(&self, message: &str);

    /// Display error message
    fn error(&self, message: &str);

    /// Display informational message
    fn info(&self, message: &str);
}
