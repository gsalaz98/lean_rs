use crate::data::DataEvent;

pub mod binary;
pub mod csv;

pub trait DataFeed<T>
where
    T: DataEvent
{
    fn cache();

    fn read_from_cache() -> T;
}