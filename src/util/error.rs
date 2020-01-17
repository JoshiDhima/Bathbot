use rosu::backend::OsuError;
use serenity::{framework::standard::CommandError, Error as SerenityError};
use std::{env, fmt, io, num};

#[derive(Debug)]
pub enum Error {
    Custom(String),
    Command(CommandError),
    ParseInt(num::ParseIntError),
    Io(io::Error),
    Serenity(SerenityError),
    Env(env::VarError),
    Osu(OsuError),
}

impl From<CommandError> for Error {
    fn from(e: CommandError) -> Self {
        Self::Command(e)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<serenity::Error> for Error {
    fn from(e: SerenityError) -> Self {
        Self::Serenity(e)
    }
}

impl From<env::VarError> for Error {
    fn from(e: env::VarError) -> Self {
        Self::Env(e)
    }
}

impl From<OsuError> for Error {
    fn from(e: OsuError) -> Self {
        Self::Osu(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Custom(e) => write!(f, "{}", e),
            Self::Command(e) => write!(f, "{:?}", e),
            Self::ParseInt(e) => write!(f, "{}", e),
            Self::Io(e) => write!(f, "{}", e),
            Self::Serenity(e) => write!(f, "{}", e),
            Self::Env(e) => write!(f, "{}", e),
            Self::Osu(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {}