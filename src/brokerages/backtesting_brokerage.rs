
pub trait Brokerage {
    type ConnectionStatus;

    fn get_connection_status() -> Self::ConnectionStatus;
}