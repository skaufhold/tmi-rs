(function() {var implementors = {};
implementors["futures_channel"] = [{text:"impl&lt;T&gt; <a class=\"trait\" href=\"futures_core/stream/trait.FusedStream.html\" title=\"trait futures_core::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_channel/mpsc/struct.Receiver.html\" title=\"struct futures_channel::mpsc::Receiver\">Receiver</a>&lt;T&gt;",synthetic:false,types:["futures_channel::mpsc::Receiver"]},{text:"impl&lt;T&gt; <a class=\"trait\" href=\"futures_core/stream/trait.FusedStream.html\" title=\"trait futures_core::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_channel/mpsc/struct.UnboundedReceiver.html\" title=\"struct futures_channel::mpsc::UnboundedReceiver\">UnboundedReceiver</a>&lt;T&gt;",synthetic:false,types:["futures_channel::mpsc::UnboundedReceiver"]},];
implementors["futures_core"] = [];
implementors["tmi_rs"] = [{text:"impl&lt;St&gt; <a class=\"trait\" href=\"futures_core/stream/trait.FusedStream.html\" title=\"trait futures_core::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"tmi_rs/event/struct.TwitchChatStream.html\" title=\"struct tmi_rs::event::TwitchChatStream\">TwitchChatStream</a>&lt;St&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;St: <a class=\"trait\" href=\"futures_core/stream/trait.FusedStream.html\" title=\"trait futures_core::stream::FusedStream\">FusedStream</a> + <a class=\"trait\" href=\"futures_core/stream/trait.Stream.html\" title=\"trait futures_core::stream::Stream\">Stream</a>&lt;Item = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"enum\" href=\"tungstenite/protocol/message/enum.Message.html\" title=\"enum tungstenite::protocol::message::Message\">Message</a>, <a class=\"enum\" href=\"tungstenite/error/enum.Error.html\" title=\"enum tungstenite::error::Error\">WsError</a>&gt;&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>",synthetic:false,types:["tmi_rs::event::stream::TwitchChatStream"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        })()