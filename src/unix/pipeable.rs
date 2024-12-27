pub trait UnixPipeable {
    fn get_fd(&self) -> i32;
}
