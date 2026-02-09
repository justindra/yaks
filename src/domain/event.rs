// Event domain model - represents a logged yak operation

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Event {
    pub operation: String,
    pub args: Vec<String>,
    pub stdin: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub author: String,
}

impl Event {
    #[allow(dead_code)]
    pub fn new(
        operation: String,
        args: Vec<String>,
        stdin: Option<String>,
        timestamp: DateTime<Utc>,
        author: String,
    ) -> Self {
        Self {
            operation,
            args,
            stdin,
            timestamp,
            author,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let timestamp = Utc::now();
        let event = Event::new(
            "add".to_string(),
            vec!["test yak".to_string()],
            None,
            timestamp,
            "user@example.com".to_string(),
        );

        assert_eq!(event.operation, "add");
        assert_eq!(event.args, vec!["test yak".to_string()]);
        assert_eq!(event.stdin, None);
        assert_eq!(event.timestamp, timestamp);
        assert_eq!(event.author, "user@example.com");
    }

    #[test]
    fn test_event_with_stdin() {
        let timestamp = Utc::now();
        let event = Event::new(
            "context".to_string(),
            vec!["test yak".to_string()],
            Some("This is context".to_string()),
            timestamp,
            "user@example.com".to_string(),
        );

        assert_eq!(event.operation, "context");
        assert_eq!(event.stdin, Some("This is context".to_string()));
    }
}
