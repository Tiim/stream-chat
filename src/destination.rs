use tokio::task::JoinHandle;

pub trait Dest {
  fn run(self)->JoinHandle<()>;
}
