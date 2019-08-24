use crate::data::Slice;
use crate::algorithm::qc_algorithm::QCAlgorithm;

#[derive(QCAlgorithm)]
pub struct BasicTemplateAlgorithm();

impl QCAlgorithm for BasicTemplateAlgorithm {
    fn initialize(&self) {

    }

    fn on_data(&self, slice: &Slice) {
        
    }
}