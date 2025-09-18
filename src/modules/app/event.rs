use crossterm::event::{Event as CEvent, KeyEvent};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

/// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event<I> {
    /// Input event.
    Input(I),
    /// Tick event.
    Tick,
}

/// Event handler.
#[allow(dead_code)]
pub struct EventHandler {
    /// Event sender channel.
    sender: mpsc::Sender<Event<KeyEvent>>,
    /// Event receiver channel.
    receiver: mpsc::Receiver<Event<KeyEvent>>,
    /// Event handler thread.
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: Duration) -> Self {
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if crossterm::event::poll(timeout).expect("no events available")
                        && let CEvent::Key(key) =
                            crossterm::event::read().expect("unable to read event")
                        {
                            sender
                                .send(Event::Input(key))
                                .expect("failed to send event");
                        }

                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there are no events available and will never return an error.
    pub fn next(&self) -> Event<KeyEvent> {
        self.receiver.recv().expect("failed to receive event")
    }
}
