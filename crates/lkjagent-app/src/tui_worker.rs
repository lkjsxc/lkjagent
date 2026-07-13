use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, SyncSender, TryRecvError, TrySendError};
use std::thread::{self, JoinHandle};

use crate::model_io::Endpoint;

enum Command {
    Wake,
    Stop,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerNotice {
    CycleFinished,
}

pub struct Worker {
    commands: SyncSender<Command>,
    notices: Receiver<WorkerNotice>,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn start(data_dir: &Path) -> Result<Self, String> {
        let data = data_dir.to_path_buf();
        Self::spawn_with(data_dir, move || {
            Box::new(crate::endpoint::LlmEndpoint::new(&data))
        })
    }

    pub fn spawn_with<F>(data_dir: &Path, factory: F) -> Result<Self, String>
    where
        F: FnOnce() -> Box<dyn Endpoint> + Send + 'static,
    {
        let (command_tx, command_rx) = mpsc::sync_channel(1);
        let (notice_tx, notice_rx) = mpsc::sync_channel(8);
        let data = PathBuf::from(data_dir);
        let thread = thread::Builder::new()
            .name("lkjagent-tui-worker".to_string())
            .spawn(move || worker_loop(data, factory(), command_rx, notice_tx))
            .map_err(|error| error.to_string())?;
        let worker = Self {
            commands: command_tx,
            notices: notice_rx,
            thread: Some(thread),
        };
        worker.wake();
        Ok(worker)
    }

    pub fn wake(&self) {
        match self.commands.try_send(Command::Wake) {
            Ok(()) | Err(TrySendError::Full(Command::Wake)) => {}
            Err(TrySendError::Disconnected(Command::Wake)) => {}
            Err(TrySendError::Full(Command::Stop))
            | Err(TrySendError::Disconnected(Command::Stop)) => {}
        }
    }

    pub fn drain(&self) -> usize {
        let mut count = 0;
        loop {
            match self.notices.try_recv() {
                Ok(WorkerNotice::CycleFinished) => count += 1,
                Err(TryRecvError::Empty | TryRecvError::Disconnected) => return count,
            }
        }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        let _ = self.commands.try_send(Command::Stop);
        if self.thread.as_ref().is_some_and(JoinHandle::is_finished) {
            if let Some(thread) = self.thread.take() {
                let _ = thread.join();
            }
        }
    }
}

fn worker_loop(
    data_dir: PathBuf,
    mut endpoint: Box<dyn Endpoint>,
    commands: Receiver<Command>,
    notices: SyncSender<WorkerNotice>,
) {
    while let Ok(command) = commands.recv() {
        match command {
            Command::Stop => return,
            Command::Wake => {
                let _ = crate::public_loop::run_once(&data_dir, endpoint.as_mut());
                let _ = notices.try_send(WorkerNotice::CycleFinished);
            }
        }
    }
}
