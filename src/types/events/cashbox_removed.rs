use crate::{impl_default, std::fmt, Error, ResponseStatus, Result};

use super::Method;

/// Represents a [CashboxRemoved](crate::ResponseStatus::CashboxRemoved) event.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CashboxRemovedEvent;

impl CashboxRemovedEvent {
    /// Creates a new [CashboxRemovedEvent].
    pub const fn new() -> Self {
        Self {}
    }

    /// Gets the [Method] for the [CashboxRemovedEvent].
    pub const fn method() -> Method {
        Method::CashboxRemoved
    }

    /// Converts the [CashboxRemovedEvent] to a string.
    pub const fn to_str(&self) -> &'static str {
        Self::method().to_str()
    }

    /// Gets the length of the event in a [PollResponse](crate::PollResponse).
    pub const fn len() -> usize {
        1
    }
}

impl TryFrom<&[u8]> for CashboxRemovedEvent {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        if val.is_empty() {
            Err(Error::InvalidLength((val.len(), 1)))
        } else {
            match ResponseStatus::from(val[0]) {
                ResponseStatus::CashboxRemoved => Ok(Self::new()),
                event => Err(Error::InvalidEvent((event, ResponseStatus::CashboxRemoved))),
            }
        }
    }
}

impl<const N: usize> TryFrom<[u8; N]> for CashboxRemovedEvent {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for CashboxRemovedEvent {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl From<&CashboxRemovedEvent> for &'static str {
    fn from(val: &CashboxRemovedEvent) -> Self {
        val.to_str()
    }
}

impl From<CashboxRemovedEvent> for &'static str {
    fn from(val: CashboxRemovedEvent) -> Self {
        (&val).into()
    }
}

impl fmt::Display for CashboxRemovedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{"{}"}}"#, self.to_str())
    }
}

impl_default!(CashboxRemovedEvent);
