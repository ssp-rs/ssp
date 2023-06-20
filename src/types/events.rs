//! Event types for polling responses.

use crate::{std::fmt, Error};

mod cashbox_removed;
mod cashbox_replaced;
mod disable;
mod disabled;
mod enable;
mod fraud_attempt;
mod method;
mod note_cleared_from_front;
mod note_cleared_into_cashbox;
mod note_credit;
mod read;
mod reject;
mod rejected;
mod rejecting;
mod reset;
mod stack;
mod stacked;
mod stacker_full;
mod stacking;
mod status;
mod unsafe_jam;

pub use cashbox_removed::*;
pub use cashbox_replaced::*;
pub use disable::*;
pub use disabled::*;
pub use enable::*;
pub use fraud_attempt::*;
pub use method::*;
pub use note_cleared_from_front::*;
pub use note_cleared_into_cashbox::*;
pub use note_credit::*;
pub use read::*;
pub use reject::*;
pub use rejected::*;
pub use rejecting::*;
pub use reset::*;
pub use stack::*;
pub use stacked::*;
pub use stacker_full::*;
pub use stacking::*;
pub use status::*;
pub use unsafe_jam::*;

pub const OPEN_BRACE: &str = "{";
pub const CLOSE_BRACE: &str = "}";

/// JSON-RPC payloads for request parameters and response results.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum EventPayload {
    Error(Error),
    // Command event payloads
    DisableEvent(DisableEvent),
    EnableEvent(EnableEvent),
    RejectEvent(RejectEvent),
    StackEvent(StackEvent),
    StatusEvent(StatusEvent),
    // Response event payloads
    CashboxRemovedEvent(CashboxRemovedEvent),
    CashboxReplacedEvent(CashboxReplacedEvent),
    DisabledEvent(DisabledEvent),
    FraudAttemptEvent(FraudAttemptEvent),
    NoteClearedFromFrontEvent(NoteClearedFromFrontEvent),
    NoteClearedIntoCashboxEvent(NoteClearedIntoCashboxEvent),
    NoteCreditEvent(NoteCreditEvent),
    ReadEvent(ReadEvent),
    RejectedEvent(RejectedEvent),
    RejectingEvent(RejectingEvent),
    // Reset is a command & response event payload
    ResetEvent(ResetEvent),
    StackedEvent(StackedEvent),
    StackerFullEvent(StackerFullEvent),
    StackingEvent(StackingEvent),
    UnsafeJamEvent(UnsafeJamEvent),
}

impl EventPayload {
    pub fn method(&self) -> Method {
        match self {
            Self::Error(_) => Method::Fail,
            Self::DisableEvent(_) => DisableEvent::method(),
            Self::EnableEvent(_) => EnableEvent::method(),
            Self::RejectEvent(_) => RejectEvent::method(),
            Self::StackEvent(_) => StackEvent::method(),
            Self::StatusEvent(_) => StatusEvent::method(),
            Self::CashboxRemovedEvent(_) => CashboxRemovedEvent::method(),
            Self::CashboxReplacedEvent(_) => CashboxReplacedEvent::method(),
            Self::DisabledEvent(_) => DisabledEvent::method(),
            Self::FraudAttemptEvent(_) => FraudAttemptEvent::method(),
            Self::NoteClearedFromFrontEvent(_) => NoteClearedFromFrontEvent::method(),
            Self::NoteClearedIntoCashboxEvent(_) => NoteClearedIntoCashboxEvent::method(),
            Self::NoteCreditEvent(_) => NoteCreditEvent::method(),
            Self::ReadEvent(_) => ReadEvent::method(),
            Self::RejectedEvent(_) => RejectedEvent::method(),
            Self::RejectingEvent(_) => RejectingEvent::method(),
            Self::ResetEvent(_) => ResetEvent::method(),
            Self::StackedEvent(_) => StackedEvent::method(),
            Self::StackerFullEvent(_) => StackerFullEvent::method(),
            Self::StackingEvent(_) => StackingEvent::method(),
            Self::UnsafeJamEvent(_) => UnsafeJamEvent::method(),
        }
    }
}

impl fmt::Display for EventPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error(err) => write!(f, "{OPEN_BRACE}\"error\": \"{err}\"{CLOSE_BRACE}"),
            Self::DisableEvent(evt) => write!(f, "{evt}"),
            Self::EnableEvent(evt) => write!(f, "{evt}"),
            Self::RejectEvent(evt) => write!(f, "{evt}"),
            Self::StackEvent(evt) => write!(f, "{evt}"),
            Self::StatusEvent(evt) => write!(f, "{evt}"),
            Self::CashboxRemovedEvent(evt) => write!(f, "{evt}"),
            Self::CashboxReplacedEvent(evt) => write!(f, "{evt}"),
            Self::DisabledEvent(evt) => write!(f, "{evt}"),
            Self::FraudAttemptEvent(evt) => write!(f, "{evt}"),
            Self::NoteClearedFromFrontEvent(evt) => write!(f, "{evt}"),
            Self::NoteClearedIntoCashboxEvent(evt) => write!(f, "{evt}"),
            Self::NoteCreditEvent(evt) => write!(f, "{evt}"),
            Self::ReadEvent(evt) => write!(f, "{evt}"),
            Self::RejectedEvent(evt) => write!(f, "{evt}"),
            Self::RejectingEvent(evt) => write!(f, "{evt}"),
            Self::ResetEvent(evt) => write!(f, "{evt}"),
            Self::StackedEvent(evt) => write!(f, "{evt}"),
            Self::StackerFullEvent(evt) => write!(f, "{evt}"),
            Self::StackingEvent(evt) => write!(f, "{evt}"),
            Self::UnsafeJamEvent(evt) => write!(f, "{evt}"),
        }
    }
}

macro_rules! from_event_for_payload {
    ($event:ident) => {
        impl From<$event> for EventPayload {
            fn from(val: $event) -> Self {
                Self::$event(val)
            }
        }

        impl From<&$event> for EventPayload {
            fn from(val: &$event) -> Self {
                Self::$event(*val)
            }
        }

        impl TryFrom<EventPayload> for $event {
            type Error = $crate::Error;

            fn try_from(val: EventPayload) -> $crate::Result<Self> {
                match val {
                    EventPayload::$event(evt) => Ok(evt),
                    event => Err(Error::InvalidEvent((
                        event.method().into(),
                        $event::method().into(),
                    ))),
                }
            }
        }

        impl TryFrom<&EventPayload> for $event {
            type Error = $crate::Error;

            fn try_from(val: &EventPayload) -> $crate::Result<Self> {
                $event::try_from(val.clone())
            }
        }
    };
}

// Command events
from_event_for_payload!(DisableEvent);
from_event_for_payload!(EnableEvent);
from_event_for_payload!(RejectEvent);
from_event_for_payload!(StackEvent);
from_event_for_payload!(StatusEvent);
// Response events
from_event_for_payload!(CashboxRemovedEvent);
from_event_for_payload!(CashboxReplacedEvent);
from_event_for_payload!(DisabledEvent);
from_event_for_payload!(FraudAttemptEvent);
from_event_for_payload!(NoteClearedFromFrontEvent);
from_event_for_payload!(NoteClearedIntoCashboxEvent);
from_event_for_payload!(NoteCreditEvent);
from_event_for_payload!(ReadEvent);
from_event_for_payload!(RejectedEvent);
from_event_for_payload!(RejectingEvent);
from_event_for_payload!(ResetEvent); // Reset is also a command
from_event_for_payload!(StackedEvent);
from_event_for_payload!(StackerFullEvent);
from_event_for_payload!(StackingEvent);
from_event_for_payload!(UnsafeJamEvent);

/// Represents a generic event from a polling response.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Event {
    method: Method,
    payload: EventPayload,
}

impl Event {
    /// Creates a new [Event] from the provided `method` and `data`.
    pub fn new(method: Method, payload: EventPayload) -> Self {
        Self { method, payload }
    }

    /// Gets the method.
    pub const fn method(&self) -> Method {
        self.method
    }

    /// Gets the method as a `&str`.
    pub const fn method_str(&self) -> &str {
        self.method.to_str()
    }

    /// Sets the method.
    pub fn set_method(&mut self, method: Method) {
        self.method = method;
    }

    /// Gets a reference to the [EventPayload].
    pub fn payload(&self) -> &EventPayload {
        &self.payload
    }

    /// Sets the [EventPayload].
    pub fn set_payload(&mut self, payload: EventPayload) {
        self.payload = payload
    }
}

impl From<Method> for Event {
    fn from(method: Method) -> Self {
        let payload = match method {
            Method::Fail => EventPayload::Error(Error::Generic(-1)),
            Method::Disable | Method::Shutdown => {
                EventPayload::DisableEvent(DisableEvent::default())
            }
            Method::Enable => EventPayload::EnableEvent(EnableEvent::default()),
            Method::Reject => EventPayload::RejectEvent(RejectEvent::default()),
            Method::Stack => EventPayload::StackEvent(StackEvent::default()),
            Method::Status => EventPayload::StatusEvent(StatusEvent::default()),
            Method::CashboxRemoved => {
                EventPayload::CashboxRemovedEvent(CashboxRemovedEvent::default())
            }
            Method::CashboxReplaced => {
                EventPayload::CashboxReplacedEvent(CashboxReplacedEvent::default())
            }
            Method::Disabled => EventPayload::DisabledEvent(DisabledEvent::default()),
            Method::FraudAttempt => EventPayload::FraudAttemptEvent(FraudAttemptEvent::default()),
            Method::NoteClearedFromFront => {
                EventPayload::NoteClearedFromFrontEvent(NoteClearedFromFrontEvent::default())
            }
            Method::NoteClearedIntoCashbox => {
                EventPayload::NoteClearedIntoCashboxEvent(NoteClearedIntoCashboxEvent::default())
            }
            Method::NoteCredit => EventPayload::NoteCreditEvent(NoteCreditEvent::default()),
            Method::Read => EventPayload::ReadEvent(ReadEvent::default()),
            Method::Rejected => EventPayload::RejectedEvent(RejectedEvent::default()),
            Method::Rejecting => EventPayload::RejectingEvent(RejectingEvent::default()),
            Method::Reset => EventPayload::ResetEvent(ResetEvent::default()),
            Method::Stacked => EventPayload::StackedEvent(StackedEvent::default()),
            Method::StackerFull => EventPayload::StackerFullEvent(StackerFullEvent::default()),
            Method::Stacking => EventPayload::StackingEvent(StackingEvent::default()),
            Method::UnsafeJam => EventPayload::UnsafeJamEvent(UnsafeJamEvent::default()),
            Method::Reserved(m) => EventPayload::Error(Error::Generic(-(m as i64))),
        };

        Self { method, payload }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method = self.method();
        let payload = self.payload();

        write!(f, "method: {method}, payload: {payload}")
    }
}

macro_rules! from_event_for_event {
    ($event:ident) => {
        impl From<&$event> for Event {
            fn from(val: &$event) -> Self {
                Self::new($event::method(), val.into())
            }
        }

        impl From<$event> for Event {
            fn from(val: $event) -> Self {
                (&val).into()
            }
        }

        impl TryFrom<&Event> for $event {
            type Error = $crate::Error;

            fn try_from(val: &Event) -> $crate::Result<Self> {
                $event::try_from(val.payload())
            }
        }

        impl TryFrom<Event> for $event {
            type Error = $crate::Error;

            fn try_from(val: Event) -> $crate::Result<Self> {
                $event::try_from(val.payload())
            }
        }
    };
}

// Command events
from_event_for_event!(DisableEvent);
from_event_for_event!(EnableEvent);
from_event_for_event!(RejectEvent);
from_event_for_event!(StackEvent);
from_event_for_event!(StatusEvent);
// Response events
from_event_for_event!(CashboxRemovedEvent);
from_event_for_event!(CashboxReplacedEvent);
from_event_for_event!(DisabledEvent);
from_event_for_event!(FraudAttemptEvent);
from_event_for_event!(NoteClearedFromFrontEvent);
from_event_for_event!(NoteClearedIntoCashboxEvent);
from_event_for_event!(NoteCreditEvent);
from_event_for_event!(ReadEvent);
from_event_for_event!(RejectedEvent);
from_event_for_event!(RejectingEvent);
from_event_for_event!(ResetEvent); // Reset is also a command
from_event_for_event!(StackedEvent);
from_event_for_event!(StackerFullEvent);
from_event_for_event!(StackingEvent);
from_event_for_event!(UnsafeJamEvent);