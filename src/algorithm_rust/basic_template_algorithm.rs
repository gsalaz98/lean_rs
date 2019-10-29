use std::collections::HashMap;

use crate::data::{BaseData, Slice, Symbol};
use crate::algorithm::qc_algorithm::QCAlgorithm;

pub struct BasicTemplateAlgorithm;

impl QCAlgorithm for BasicTemplateAlgorithm {
    fn initialize(&self) {
        self.log("Hello from initialize!");
    }

    fn on_data(&self, data: HashMap<&Symbol, &dyn BaseData>) {
        
    }
}