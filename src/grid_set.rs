
#[derive(Debug, Copy, Clone)]
pub enum NodeState {
    Start,
    End,
    Blank,
    Obstacle,
    Path,
    Open,
    Closed,
}

#[derive(Debug, Clone, Copy)]
pub struct Node  {
    pub state: NodeState,
    // ...
}
