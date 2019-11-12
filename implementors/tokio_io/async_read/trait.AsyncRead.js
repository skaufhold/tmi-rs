(function() {var implementors = {};
implementors["tokio_net"] = [{text:"impl&lt;E&gt; <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"struct\" href=\"tokio_net/util/struct.PollEvented.html\" title=\"struct tokio_net::util::PollEvented\">PollEvented</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"mio/event_imp/trait.Evented.html\" title=\"trait mio::event_imp::Evented\">Evented</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>",synthetic:false,types:["tokio_net::util::poll_evented::PollEvented"]},{text:"impl&lt;'_&gt; <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"struct\" href=\"tokio_net/tcp/split/struct.ReadHalf.html\" title=\"struct tokio_net::tcp::split::ReadHalf\">ReadHalf</a>&lt;'_&gt;",synthetic:false,types:["tokio_net::tcp::split::ReadHalf"]},{text:"impl <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"struct\" href=\"tokio_net/tcp/struct.TcpStream.html\" title=\"struct tokio_net::tcp::TcpStream\">TcpStream</a>",synthetic:false,types:["tokio_net::tcp::stream::TcpStream"]},{text:"impl&lt;'_&gt; <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"struct\" href=\"tokio_net/uds/split/struct.ReadHalf.html\" title=\"struct tokio_net::uds::split::ReadHalf\">ReadHalf</a>&lt;'_&gt;",synthetic:false,types:["tokio_net::uds::split::ReadHalf"]},{text:"impl <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"struct\" href=\"tokio_net/uds/struct.UnixStream.html\" title=\"struct tokio_net::uds::UnixStream\">UnixStream</a>",synthetic:false,types:["tokio_net::uds::stream::UnixStream"]},];
implementors["tokio_tls"] = [{text:"impl&lt;S&gt; <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"struct\" href=\"tokio_tls/struct.TlsStream.html\" title=\"struct tokio_tls::TlsStream\">TlsStream</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"tokio_io/async_write/trait.AsyncWrite.html\" title=\"trait tokio_io::async_write::AsyncWrite\">AsyncWrite</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>",synthetic:false,types:["tokio_tls::TlsStream"]},];
implementors["tokio_tungstenite"] = [{text:"impl&lt;S:&nbsp;<a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>, T:&nbsp;<a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>&gt; <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"enum\" href=\"tokio_tungstenite/stream/enum.Stream.html\" title=\"enum tokio_tungstenite::stream::Stream\">Stream</a>&lt;S, T&gt;",synthetic:false,types:["tokio_tungstenite::stream::Stream"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        })()