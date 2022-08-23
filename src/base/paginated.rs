pub trait Paginated {
    fn get_next_uri(&self) -> Option<String>;
    fn get_data<T, P>(&self) -> Vec<P>;
}