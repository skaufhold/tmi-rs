use std::borrow::Borrow;
use std::fmt::Debug;

use fnv::FnvHashMap;

use crate::events::Event;
use crate::util::RefToString;
use crate::{Error, StringRef};

/// Converts events from references into owned versions of themselves
pub trait ToOwnedEvent {
    type Owned;
    /// Convert the event to its owned version
    fn to_owned_event(&self) -> Self::Owned;
}

impl<T: Copy> ToOwnedEvent for T {
    type Owned = T;
    fn to_owned_event(&self) -> Self::Owned {
        *self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventContent<T, Inner>
where
    T: StringRef,
    Inner: Debug + Clone + Eq,
{
    pub(crate) sender: Option<T>,
    pub(crate) event: Inner,
    pub(crate) tags: Option<FnvHashMap<T, T>>,
}

/// Methods common to all EventContent variants
impl<T, Inner> EventContent<T, Inner>
where
    T: StringRef,
    Inner: Debug + Clone + Eq,
    Event<T>: From<EventContent<T, Inner>>,
{
    /// Get the sender of the message
    pub fn sender(&self) -> &Option<T> {
        &self.sender
    }

    /// Get the data of the inner event type
    pub fn event(&self) -> &Inner {
        &self.event
    }

    /// Get the map of all IRCv3 tags
    pub fn tags(&self) -> &Option<FnvHashMap<T, T>> {
        &self.tags
    }

    /// Get a tag value from the message by its key
    pub fn tag<Q: Borrow<str>>(&self, key: Q) -> Option<&T> {
        self.tags.as_ref().and_then(|tags| tags.get(key.borrow()))
    }

    /// Gets a tag value, returns an Error if the value is not set. Intended for use in
    /// internal tag accessor functions where the tag should always be available
    pub(crate) fn required_tag<Q: Borrow<str>>(&self, key: Q) -> Result<&T, Error> {
        self.tag(key.borrow()).ok_or_else(|| Error::MissingTag {
            tag: key.borrow().to_string().into(),
            event: (&Event::<T>::from(self.clone())).into(),
        })
    }
}

/// Generic ToOwned implementation for all EventContent variants
impl<T, Inner> ToOwnedEvent for EventContent<T, Inner>
where
    T: StringRef,
    Inner: Debug + Clone + Eq + ToOwnedEvent,
    Inner::Owned: Debug + Clone + Eq,
{
    type Owned = EventContent<String, Inner::Owned>;

    fn to_owned_event(&self) -> Self::Owned {
        EventContent {
            sender: self.sender.as_ref().map(RefToString::ref_to_string),
            event: self.event.to_owned_event(),
            tags: self.tags.as_ref().map(|hash_map| {
                hash_map
                    .iter()
                    .map(|(key, val)| (key.ref_to_string(), val.ref_to_string()))
                    .collect::<FnvHashMap<String, String>>()
            }),
        }
    }
}

/// Welcome messages that Twitch sends after connection and logging
/// in successfully
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectMessageEvent<T: Debug + Clone + Eq> {
    pub command: T,
    pub params: Vec<T>,
}

impl<T: StringRef> ToOwnedEvent for ConnectMessageEvent<T> {
    type Owned = ConnectMessageEvent<String>;

    fn to_owned_event(&self) -> Self::Owned {
        ConnectMessageEvent {
            command: self.command.ref_to_string(),
            params: self.params.iter().map(RefToString::ref_to_string).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserEvent<T: StringRef> {
    pub user: T,
}

impl<T: StringRef> ToOwnedEvent for UserEvent<T> {
    type Owned = UserEvent<String>;

    fn to_owned_event(&self) -> Self::Owned {
        UserEvent {
            user: self.user.ref_to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelMessageEvent<T: StringRef> {
    pub channel: T,
    pub message: T,
}

pub trait ChannelMessageData<T> {
    /// Get the channel this message was sent from
    fn channel(&self) -> &T;
    /// Get the message
    fn message(&self) -> &T;
}

impl<T, U> ChannelMessageData<T> for EventContent<T, U>
where
    T: StringRef,
    U: Debug + Clone + Eq + AsRef<ChannelMessageEvent<T>>,
{
    #[inline]
    fn channel(&self) -> &T {
        &self.event.as_ref().channel
    }

    #[inline]
    fn message(&self) -> &T {
        &self.event.as_ref().message
    }
}

impl<T: StringRef> ToOwnedEvent for ChannelMessageEvent<T> {
    type Owned = ChannelMessageEvent<String>;

    fn to_owned_event(&self) -> Self::Owned {
        ChannelMessageEvent {
            channel: self.channel.ref_to_string(),
            message: self.message.ref_to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelEvent<T: StringRef> {
    pub channel: T,
}

impl<T: StringRef> ToOwnedEvent for ChannelEvent<T> {
    type Owned = ChannelEvent<String>;

    fn to_owned_event(&self) -> Self::Owned {
        ChannelEvent {
            channel: self.channel.ref_to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelUserEvent<T: StringRef> {
    pub channel: T,
    pub user: Option<T>,
}

impl<T: StringRef> ToOwnedEvent for ChannelUserEvent<T> {
    type Owned = ChannelUserEvent<String>;

    fn to_owned_event(&self) -> Self::Owned {
        ChannelUserEvent {
            channel: self.channel.ref_to_string(),
            user: self.user.as_ref().map(RefToString::ref_to_string),
        }
    }
}

macro_rules! impl_inner_to_owned {
    ($type:ident, $inner:ident) => {
        impl<T: StringRef> ToOwnedEvent for $type<T> {
            type Owned = $type<String>;

            fn to_owned_event(&self) -> Self::Owned {
                $type(self.0.to_owned_event())
            }
        }

        impl<'a, T: StringRef> AsRef<$inner<T>> for $type<T> {
            fn as_ref(&self) -> &$inner<T> {
                &self.0
            }
        }
    };
}

#[derive(Debug, Clone, Eq, PartialEq, From, Into)]
pub struct PrivMsgEvent<T: StringRef>(ChannelMessageEvent<T>);
impl_inner_to_owned!(PrivMsgEvent, ChannelMessageEvent);

#[derive(Debug, Clone, Eq, PartialEq, From, Into)]
pub struct JoinEvent<T: StringRef>(ChannelEvent<T>);
impl_inner_to_owned!(JoinEvent, ChannelEvent);

#[derive(Debug, Clone, Eq, PartialEq, From, Into)]
pub struct EndOfNamesEvent<T: StringRef>(ChannelEvent<T>);
impl_inner_to_owned!(EndOfNamesEvent, ChannelEvent);

#[derive(Debug, Clone, Eq, PartialEq, From, Into)]
pub struct PartEvent<T: StringRef>(ChannelEvent<T>);
impl_inner_to_owned!(PartEvent, ChannelEvent);

#[derive(Debug, Clone, Eq, PartialEq, From, Into)]
pub struct ClearChatEvent<T: StringRef>(ChannelUserEvent<T>);
impl_inner_to_owned!(ClearChatEvent, ChannelUserEvent);

#[derive(Debug, Clone, Eq, PartialEq, From, Into)]
pub struct ClearMsgEvent<T: StringRef>(ChannelMessageEvent<T>);
impl_inner_to_owned!(ClearMsgEvent, ChannelMessageEvent);

impl<T: StringRef> EventContent<T, ClearMsgEvent<T>> {
    #[inline]
    pub fn target_msg_id(&self) -> Result<&T, Error> {
        self.required_tag("target-msg-id")
    }

    #[inline]
    pub fn login(&self) -> Result<&T, Error> {
        self.required_tag("login")
    }
}

#[derive(Debug, Clone, Eq, PartialEq, From, Into)]
pub struct NoticeEvent<T: StringRef>(ChannelMessageEvent<T>);
impl_inner_to_owned!(NoticeEvent, ChannelMessageEvent);

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct ReconnectEvent;

#[derive(Debug, Clone, Eq, PartialEq, From, Into)]
pub struct RoomStateEvent<T: StringRef>(ChannelEvent<T>);
impl_inner_to_owned!(RoomStateEvent, ChannelEvent);

#[derive(Debug, Clone, Eq, PartialEq, From, Into)]
pub struct UserNoticeEvent<T: StringRef>(ChannelMessageEvent<T>);
impl_inner_to_owned!(UserNoticeEvent, ChannelMessageEvent);

#[derive(Debug, Clone, Eq, PartialEq, From, Into)]
pub struct UserStateEvent<T: StringRef>(ChannelEvent<T>);
impl_inner_to_owned!(UserStateEvent, ChannelEvent);

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct GlobalUserStateEvent;

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct CloseEvent;

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct PingEvent;

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct PongEvent;

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct UnknownEvent;

/// NAMES list response data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamesListEvent<T: StringRef> {
    pub user: T,
    pub channel: T,
    pub names: Vec<T>,
}

impl<T: StringRef> ToOwnedEvent for NamesListEvent<T> {
    type Owned = NamesListEvent<String>;

    fn to_owned_event(&self) -> Self::Owned {
        NamesListEvent {
            user: self.user.ref_to_string(),
            channel: self.channel.ref_to_string(),
            names: self.names.iter().map(RefToString::ref_to_string).collect(),
        }
    }
}

/// User mode change event
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModeChangeEvent<T: StringRef> {
    pub channel: T,
    pub mode_change: T,
    pub user: T,
}

impl<T: StringRef> ToOwnedEvent for ModeChangeEvent<T> {
    type Owned = ModeChangeEvent<String>;

    fn to_owned_event(&self) -> Self::Owned {
        ModeChangeEvent {
            channel: self.channel.ref_to_string(),
            mode_change: self.mode_change.ref_to_string(),
            user: self.user.ref_to_string(),
        }
    }
}

/// Whisper message event data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhisperEvent<T: StringRef> {
    pub recipient: T,
    pub message: T,
}

impl<T: StringRef> ToOwnedEvent for WhisperEvent<T> {
    type Owned = WhisperEvent<String>;

    fn to_owned_event(&self) -> Self::Owned {
        WhisperEvent {
            recipient: self.recipient.ref_to_string(),
            message: self.message.ref_to_string(),
        }
    }
}

/// Host event data (hosting channel, target channel, viewers)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostEvent<T: StringRef> {
    pub hosting_channel: T,
    pub target_channel: Option<T>,
    pub viewer_count: Option<usize>,
}

impl<T: StringRef> ToOwnedEvent for HostEvent<T> {
    type Owned = HostEvent<String>;

    fn to_owned_event(&self) -> Self::Owned {
        HostEvent {
            hosting_channel: self.hosting_channel.ref_to_string(),
            target_channel: self.target_channel.as_ref().map(RefToString::ref_to_string),
            viewer_count: self.viewer_count,
        }
    }
}

/// IRCv3 CAP response data, sent in response to CAP requests
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityEvent<T: Debug + Clone + Eq> {
    pub params: Vec<T>,
}

impl<T: StringRef> ToOwnedEvent for CapabilityEvent<T> {
    type Owned = CapabilityEvent<String>;

    fn to_owned_event(&self) -> Self::Owned {
        CapabilityEvent {
            params: self.params.iter().map(RefToString::ref_to_string).collect(),
        }
    }
}