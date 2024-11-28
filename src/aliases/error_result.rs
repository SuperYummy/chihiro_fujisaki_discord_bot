use std::error::Error;


pub type Hope<Desire> = Result<Desire, Box<dyn Error + Send + Sync>>;