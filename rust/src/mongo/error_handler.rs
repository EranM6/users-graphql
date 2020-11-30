pub enum MongoError {
    MissingEnvArg,
    ConnError,
    ParseError,
    NotFound,
}

impl MongoError {
    pub fn message(&self) -> &str {
        match self {
            Self::MissingEnvArg => "Missing username/password in global env",
            Self::ConnError => "Unable to connect to MongoDb",
            Self::ParseError => "Unable to parse result",
            Self::NotFound => "User not found",
        }
    }
}

impl From<std::env::VarError> for MongoError {
    fn from(e: std::env::VarError) -> Self {
        #[cfg(debug_assertions)]
        println!("{:?}", e);
        Self::MissingEnvArg
    }
}

impl From<mongodb::error::Error> for MongoError {
    fn from(e: mongodb::error::Error) -> Self {
        #[cfg(debug_assertions)]
        println!("{:?}", e);
        Self::ConnError
    }
}

impl From<mongodb::bson::de::Error> for MongoError {
    fn from(e: mongodb::bson::de::Error) -> Self {
        #[cfg(debug_assertions)]
        println!("{:?}", e);
        Self::ParseError
    }
}