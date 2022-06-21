pub trait CLI {
    type Commands;

    fn constructor(&self) -> Self::Commands;
}