use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
use futures::StreamExt;
use std::time::Duration;
use tokio::sync::mpsc;

pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Tick,
    Resize(u16, u16),
}

pub struct EventLoop {
    rx: mpsc::UnboundedReceiver<Event>,
}

impl EventLoop {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        let _tx = tx.clone();
        tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            loop {
                if let Some(Ok(event)) = reader.next().await {
                    match event {
                        CrosstermEvent::Key(key) => {
                            if _tx.send(Event::Key(key)).is_err() {
                                break;
                            }
                        }
                        CrosstermEvent::Mouse(mouse) => {
                            if _tx.send(Event::Mouse(mouse)).is_err() {
                                break;
                            }
                        }
                        CrosstermEvent::Resize(w, h) => {
                            if _tx.send(Event::Resize(w, h)).is_err() {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
        });

        let tick_tx = tx.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tick_rate);
            loop {
                interval.tick().await;
                if tick_tx.send(Event::Tick).is_err() {
                    break;
                }
            }
        });

        Self { rx }
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.rx.recv().await
    }
}
