/// Module defining all possible events we might receive from neovim

#[derive(Debug)]
pub enum Event {
    Shutdown,
    Search,
}
