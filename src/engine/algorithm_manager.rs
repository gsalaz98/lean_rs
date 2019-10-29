use std::time::Duration;

pub struct AlgorithmManager<T: QCAlgorithm> {
    previous_time: u64,
    algorithm: T,
    algorithm_id: String,
    current_time_step: u64,
    time_loop_maximum: Duration,
    data_point_count: u64,
}

impl<T> AlgorithmManager<T> where T: Algorithm {

}