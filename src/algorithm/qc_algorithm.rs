use crate::data::Slice;

pub trait QCAlgorithm {
    fn initialize(&self);
    fn on_data(&self, slice: &Slice);
}