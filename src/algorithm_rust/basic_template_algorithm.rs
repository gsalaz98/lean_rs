use crate::data::Slice;
use crate::algorithm::qc_algorithm::QCAlgorithm;

pub struct BasicTemplateAlgorithm;

impl QCAlgorithm for BasicTemplateAlgorithm {
    fn initialize(&self) {
        self.log("Hello from initialize!");
    }

    fn on_data<T>(&self, data: Vec<T>) {
        
    }
}