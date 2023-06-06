use std::ops::Range;
use std::sync::mpsc;
use tokio::sync::oneshot;

use crate::log;
use super::window;
use super::env;

pub struct WindowInfo {
    tx_window: mpsc::Sender<window::Signal>,
    tx_delay: oneshot::Sender<()>,
    handle_window: tokio::task::JoinHandle<Option<std::thread::JoinHandle<()>>>,
}

impl WindowInfo {
    pub fn new(range: Range<u32>) -> Self {
        let (signal_window, receiver_window) = std::sync::mpsc::channel();
        let (send_delay, recv_delay) = tokio::sync::oneshot::channel();
        let handle_window = tokio::spawn(async move {
            if let Err(_) = tokio::time::timeout(env::DELAY_WINDOW, recv_delay).await {
                Some(window::make_window(receiver_window, range))
            } else {
                None
            }
        });
        Self {
            tx_window: signal_window,
            tx_delay: send_delay,
            handle_window,
        }
    }
    pub fn update(&self, event: window::Signal) -> Result<(), mpsc::SendError<window::Signal>>{
        self.tx_window.send(event)
    }
    pub fn stop(self) {
        if let Err(_) = self.tx_window.send(window::Signal::Quit) {
            log!("WARN", "window receiver dropped");
        }
        if let Err(_) = self.tx_delay.send(()) {
            log!("WARN", "delay receiver dropped");
        }
        tokio::spawn(async {
            'join: {
                match self.handle_window.await {
                    Err(e) => log!("ERROR", "Error while joining window tokio thread: {}", e),
                    Ok(thr_handle) => {
                        let Some(thr_handle) = thr_handle else {break 'join;};
                        match thr_handle.join() {
                            Err(_) => log!("ERROR", "Error while joining window thread!"),
                            Ok(_) => log!("INFO", "Window thread joined!"),
                        }
                    },
                }
            }
        });
    }
}

