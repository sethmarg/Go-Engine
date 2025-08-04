use std::cmp::Ordering;
use super::*;

// Monte Carlo Tree Search Nodes
struct MCTSNode {
    parent: Option<Box<MCTSNode>>,
    state: Board,
    children: Vec<MCTSNode>,
    total_visits: u16,
    winning_visits: u16,
    score: f32,
}

// Monte Carlo Tree Search phases
impl MCTSNode {
    fn selection(&self) -> MCTSNode {
        todo!();
    }

    fn expansion(&self) {
        todo!();
    }

    fn simulation(&self) {
        todo!();
    }

    fn backpropagation(&self) {
        todo!();
    }
}

// Generates a move using this Go Engine (MCTS) to play on the given Board
pub(crate) fn generate_move(position: Board, iterations: u16) -> Move {
    let root = MCTSNode {
        parent: None,
        state: position,
        children: vec![],
        total_visits: 0,
        winning_visits: 0,
        score: 0.0,
    };

    for _ in 0..iterations {
        // todo: implement below methods and use them appropriately
        let node = root.selection();
        node.expansion();
        node.simulation();
        node.backpropagation();
    };

    if root.children.len() == 0 {
        Move::PASS
    } else {
        // todo: find child of max ending value and return its last move
        todo!()
    }
}
