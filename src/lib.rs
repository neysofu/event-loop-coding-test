/// Asynchronous, low-level event loop for a network protocol decoder.
///
/// Your task is to implement an [`Iterator`] -like event loop leveraging the
/// Rust `async` ecosystem. The event loop will trigger a new event whenever one
/// of the following happens:
///
/// 1. A new message is received from the network.
/// 2. 10 seconds have passed since the last message was received.
/// 3. 30 seconds have passed since the last message was received.
/// 4. A decoding error happened.
///
/// Each of the above must be associated with a distinct [`Event`] variant.
///
/// You may use third-party crates at your discretion.
///
/// # Further discussion points
///
/// - Are the patterns used here idiomatic? Is it possible to implement
///   [`EventLoop::next_event`] inside a trait (if so, which)?
/// - How can we test this?
/// - Considerations about allocation, [`Decoder`] buffering, and performance.
/// - What happens if we want to use a different `async` executor? Can we?
/// - What happens if we want to trigger a new event at delays other than 10 and
///   30 seconds? What happens if the list of delays is not available at
///   compile-time? In that case, all delay events may be collapsed into a single
///   [`Event`] variant, together with a [`Duration`] instance.
use futures::AsyncRead;

struct EventLoop<D, I> {
    // ...
}

impl<D, I> EventLoop<D, I>
where
    // You may add some other type constraints if you deem it necessary.
    D: Decoder,
    I: AsyncRead,
{
    /// Creates a new [`EventLoop`].
    pub fn new(decoder: D, input: I) -> Self {
        unimplemented!()
    }

    /// Waits for the next [`Event`] and then returns it. It MUST return
    /// [`None`] if the last [`Event`] was a 30 seconds delay (we're assuming
    /// the server has stopped responding).
    pub async fn next_event(&mut self) -> Option<Event> {
        unimplemented!()
    }
}

enum Event<D: Decoder> {
    // ...
}

pub trait Decoder {
    type Message;
    type Error;

    /// Ask the [`Decoder`] for some bytes in its internal buffer to store
    /// incoming data. After the whole provide slice is filled with data, and
    /// only then, you should call [`Decoder::try_decode`].
    ///
    /// The length of the returned slice is a lower bound on the amount of bytes
    /// needed to decode a full message.
    fn supply_buffer(&mut self) -> &mut [u8];

    /// Try to decode the data stored in the internal buffer.
    ///
    /// - If the data maps to a full valid message, returns `Ok(Some(message))`.
    ///   The internal buffer is automatically emptied.
    /// - If the data triggers a decoding error, returns `Err(err)`.
    /// - If the data does not trigger a decoding error, but it's not a full
    ///   message just yet, returns `Ok(None)`. Ask for more bytes via
    ///   [`Decoder::supply_buffer`].
    fn try_decode(&mut self) -> Result<Option<Self::Message>, Self::Error>;
}
