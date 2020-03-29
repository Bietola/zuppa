use futures::channel::mpsc;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Sync + Send>>;

pub type Sender<T> = mpsc::UnboundedSender<T>;
pub type Receiver<T> = mpsc::UnboundedReceiver<T>;
