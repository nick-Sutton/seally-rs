use std::cmp::Ordering;

#[derive(PartialEq)]
pub struct MinHeapNode<C> {
    pub cost: f64,
    pub config: C,
}

impl<C: PartialEq> Eq for MinHeapNode<C> {}

impl<C: PartialEq> PartialOrd for MinHeapNode<C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<C: PartialEq> Ord for MinHeapNode<C> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}