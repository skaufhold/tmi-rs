initSidebarItems({"enum":[["Event","Enum containing all event types that can be received from Twitch"]],"mod":[["tags","Convenience methods to access the IRCv3 tags twitch sends. It's possible to access the tag values using their names with the `MessageTags<T>` trait. However, there are accessor methods for all documented tags which are only implemented for the event types where they are expected to be set. Additionally, some of the methods parse the tags and return more easily processed structs instead of the raw string values (see `EmoteTag::emotes()`), `EmoteSetsTag::emote_sets()`)."]],"struct":[["CapabilityEvent","IRCv3 CAP response data, sent in response to CAP requests"],["ChannelEvent","Event containing only a channel"],["ChannelMessageEvent","Events containing a channel and a message"],["ChannelUserEvent","Event containing a channel and a username"],["ClearChatEvent","CLEARCHAT event content"],["ClearMsgEvent","CLEARMSG event content"],["CloseEvent","Connection close event"],["ConnectMessageEvent","Welcome messages that Twitch sends after connection and logging in successfully."],["EndOfNamesEvent","End of NAMES list event content"],["EventData","Content of a received message. Contains the sender, tags and and a generic `Inner` which contains the data specific to each event type."],["GlobalUserStateEvent","GLOBALUSERSTATE event"],["HostEvent","Host event data (hosting channel, target channel, viewers)"],["JoinEvent","JOIN event contents"],["ModeChangeEvent","User mode change event"],["NamesListEvent","NAMES list response data"],["NoticeEvent","NOTICE event content"],["PartEvent","PART event content"],["PingEvent","IRC PING event"],["PongEvent","IRC PONG event"],["PrivMsgEvent","PRIVMSG event contents"],["ReconnectEvent","RECONNECT event"],["RoomStateEvent","ROOMSTATE event"],["TwitchChatStream","A wrapper around the websocket stream that parses incoming IRC messages into event structs and formats Message or Command structs as IRC messages."],["UnknownEvent","Unknown event that could not be parsed."],["UserEvent","Event containing just a username"],["UserNoticeEvent","USERNOTICE event"],["UserStateEvent","USERSTATE event"],["WhisperEvent","Whisper message event data"]],"trait":[["ChannelEventData","Data accessor for channel events"],["ChannelMessageEventData","Accessors for channel message event data"],["ChannelUserEventData","Channel/user event data access"],["ConnectMessageEventData","Access to data in connect events"],["HostEventData","HOST event data accessors"],["ModeChangeData","Data accessors for MODE events"],["NamesEventData","Data accessors for NAMES events"],["ToOwnedEvent","Converts from events with inner reference types into owned versions"],["UserEventData","User event data accessors"],["WhisperEventData","Whisper event data accessors"]]});