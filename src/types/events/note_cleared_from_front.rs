use crate::{channel_value, std::fmt, ChannelValue, Error, ResponseStatus, Result};

use super::Method;

/// Represents a [NoteClearedFromFront](crate::ResponseStatus::NoteClearedFromFront) event.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NoteClearedFromFrontEvent {
    value: ChannelValue,
}

impl NoteClearedFromFrontEvent {
    /// Creates a new [NoteClearedFromFrontEvent] from the [ChannelValue].
    pub const fn new(value: ChannelValue) -> Self {
        Self { value }
    }

    /// Gets the [Method] for the [NoteClearedFromFrontEvent].
    pub const fn method() -> Method {
        Method::NoteClearedFromFront
    }

    /// Converts the [NoteClearedFromFrontEvent] to a string.
    pub const fn to_str(&self) -> &'static str {
        Self::method().to_str()
    }

    /// Gets the [ChannelValue].
    pub fn value(&self) -> &ChannelValue {
        &self.value
    }

    /// Sets the [ChannelValue].
    pub fn set_value(&mut self, value: ChannelValue) {
        self.value = value;
    }

    /// Gets the length of the event in a [PollResponse](crate::PollResponse).
    pub const fn len() -> usize {
        2
    }
}

impl TryFrom<&[u8]> for NoteClearedFromFrontEvent {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        match val.len() {
            0..=1 => Err(Error::InvalidLength((val.len(), 2))),
            _ => match ResponseStatus::from(val[0]) {
                ResponseStatus::NoteClearedFromFront => {
                    Ok(Self::new(channel_value(val[1] as usize)?))
                }
                event => Err(Error::InvalidEvent((
                    event,
                    ResponseStatus::NoteClearedFromFront,
                ))),
            },
        }
    }
}

impl<const N: usize> TryFrom<[u8; N]> for NoteClearedFromFrontEvent {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for NoteClearedFromFrontEvent {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl From<ChannelValue> for NoteClearedFromFrontEvent {
    fn from(val: ChannelValue) -> Self {
        Self::new(val)
    }
}

impl From<&ChannelValue> for NoteClearedFromFrontEvent {
    fn from(val: &ChannelValue) -> Self {
        (*val).into()
    }
}

impl From<&NoteClearedFromFrontEvent> for &'static str {
    fn from(val: &NoteClearedFromFrontEvent) -> Self {
        val.to_str()
    }
}

impl From<NoteClearedFromFrontEvent> for &'static str {
    fn from(val: NoteClearedFromFrontEvent) -> Self {
        (&val).into()
    }
}

impl fmt::Display for NoteClearedFromFrontEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method = self.to_str();
        let value = self.value();

        write!(f, r#"{{"{method}": {{"value": {value}}}}}"#)
    }
}

impl Default for NoteClearedFromFrontEvent {
    fn default() -> Self {
        Self::new(ChannelValue::default())
    }
}
