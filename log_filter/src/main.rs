pub trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

struct Filter<L, F>
where 
    L: Logger,
    F: Fn(u8, &str) -> bool
    {
    logger: L,
    filter: F,
}

impl<L, F> Filter<L, F>
where 
    L: Logger,
    F: Fn(u8, &str) -> bool
    {
    fn new(logger: L, filter: F) -> Self{
        Self { logger, filter }
    }

    fn log(&self, verbosity: u8, message: &str){
        if (self.filter)(verbosity, message){
            self.logger.log(verbosity, message);
        }
    }
}

fn main() {
    let logger = Filter::new(StderrLogger, |_verbosity, msg| msg.contains("yikes"));
    logger.log(5, "FYI");
    logger.log(1, "yikes, something went wrong");
    logger.log(2, "uhoh");
}