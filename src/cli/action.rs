use crate::cli::internal::Internal;
use {
    crate::executor::job_ref::JobRef,
    lazy_static::lazy_static,
    regex::Regex,
    serde::{de, Deserialize, Deserializer},
    std::{fmt, str::FromStr},
};

lazy_static! {
    pub static ref JOB_STRING: Regex = Regex::new(r#"^(\w+)\s*:\s*(\S+)$"#).unwrap();
}

/// an action that can be mapped to a key
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Internal(Internal),
    Job(JobRef),
}

#[derive(Debug)]
pub enum ParseActionError {
    UnknownAction(String),
    UnknowCategory(String),
    UnknownInternal(String),
}

impl fmt::Display for ParseActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnknownAction(s) => {
                write!(
                    f,
                    "Action not understood: {s:?} (did you mean \"job:{s}\"?)"
                )
            }
            Self::UnknowCategory(s) => {
                write!(f, "Unknown category: {s:?}")
            }
            Self::UnknownInternal(s) => {
                write!(f, "Internal not understood: {s:?}")
            }
        }
    }
}

impl std::error::Error for ParseActionError {}

impl FromStr for Action {
    type Err = ParseActionError;
    fn from_str(s: &str) -> Result<Self, ParseActionError> {
        if let Ok(internal) = Internal::from_str(s) {
            Ok(Self::Internal(internal))
        } else {
            Err(ParseActionError::UnknownAction(s.to_string()))
        }
    }
}

impl From<Internal> for Action {
    fn from(i: Internal) -> Self {
        Self::Internal(i)
    }
}

impl From<JobRef> for Action {
    fn from(jr: JobRef) -> Self {
        Self::Job(jr)
    }
}

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}
