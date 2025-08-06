use super::*;

/******************************************************\
|****************    PRIVATE TYPES     ****************|
\******************************************************/

// Monte Carlo Tree Search Nodes
struct MCTSNode {
    parent: Option<Box<MCTSNode>>,
    state: Board,
    children: Vec<MCTSNode>,
    total_visits: u16,
    winning_visits: u16,
    score: f64,
}

/******************************************************\
|****************        HELPER        ****************|
\******************************************************/

impl MCTSNode {
    // Scores Monte Carlo Tree nodes using the Upper Confidence for Trees formula
    fn uct_score(&self, parent: &MCTSNode) -> f64 {
        let node_wins = self.winning_visits as f64;
        let node_visits = self.total_visits as f64;
        let parent_visits = parent.total_visits as f64;
        let uct_constant = f64::sqrt(2.0);

        node_wins / node_visits + uct_constant * f64::sqrt(f64::ln(parent_visits) / node_visits)
    }
}
/*******************************************************\
|****************      TREE SEARCH      ****************|
\*******************************************************/

impl MCTSNode {
    // Selects the next node in the Monte Carlo Tree to search
    fn selection(&self) -> &MCTSNode {
        let mut best_node = self;
        let mut best_score = 0.0;
        for child in &self.children {
            let child_score = child.uct_score(self);
            if child_score > best_score {
                best_node = child;
                best_score = child_score;
            }
        }
        
        best_node
    }

    // Expands this Monte Carlo Tree Node by adding nodes to its children field
    // where a potential move has been played
    fn expansion(&self) {
        todo!();
    }

    // Simulates the result of a game played on the board this node represents
    fn simulation(&self) {
        todo!();
    }

    // Traverses up the tree from this node and updates wins accordingly
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
    }

    if root.children.len() == 0 {
        Move::PASS
    } else {
        // todo: find child of max ending value and return its last move
        todo!()
    }
}
