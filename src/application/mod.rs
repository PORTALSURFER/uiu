use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Failed to create application")]
    FailedToCreateApplication,
}

pub struct Application{}

impl Application {
    pub fn new() -> Result<Self, ApplicationError> {
        let application = Self{};
        Ok(application)
    }

    pub fn run(self) {
        loop {
            println!("Running app!");
        }
    }
}