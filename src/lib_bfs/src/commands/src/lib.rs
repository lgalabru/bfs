pub mod list_files;

pub trait Command {
    fn validate();
    fn run();
}
