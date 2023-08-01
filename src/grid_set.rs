
#[derive(Copy, Clone)]
pub enum NodeState {
    Start,
    End,
    Blank,
    Obstacle,
    Path,
    Open,
    Closed,
}

#[derive(Clone, Copy)]
pub struct Node  {
    pub state: NodeState,
    // ...
}

struct GridSet( Vec<Node> );
