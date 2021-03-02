use std::time::Instant;

/**
Time is when the event should be fired.
Expr is the cron expression
Command is the name of the external command.
Token is the authentication bearer token.
*/
struct Event {
    time: Instant,
    expr: String,
    command: String,
    token: String,
}

impl Default for Event {
    fn default() -> Self {
        let now = Instant::now();

        Self {
            time: now,
            ..Default::default()
        }
    }
}

impl Event {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
