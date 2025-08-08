use super::*;
use thunderdome::*;
/******************************************************\
|****************    PRIVATE TYPES     ****************|
\******************************************************/

// Wrapper class that contains the entire Monte Carlo Tree
struct MCTSTree {
    root_index: Index,
    arena: Arena<MCTSNode>,
}

// Monte Carlo Tree Nodes
struct MCTSNode {
    state: Board,
    played_last_move: Color,
    last_move: Option<Move>,
    parent: Option<Index>,
    children: Vec<Index>,
    total_visits: u16,
    winning_visits: u16,
    score: f64,
}

/*****************************************************\
|****************        SETUP        ****************|
\*****************************************************/

impl MCTSNode {
    // Creates a new MCTSNode with the given parameters, and setting the others to their default value
    fn new(state: Board, played_last_move: Color, last_move: Option<Move>) -> MCTSNode {
        MCTSNode {
            state,
            played_last_move,
            last_move,
            parent: None,
            children: vec![],
            total_visits: 0,
            winning_visits: 0,
            score: 0.0,
        }
    }
}

impl PartialEq for MCTSNode {
    // Custom equality function that only checks board state fields,
    // does not check any Monte Carlo related fields
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
            && self.played_last_move == other.played_last_move
            && self.last_move == other.last_move
    }
}

impl MCTSTree {
    // Creates a new MCTSTree and
    fn new(initial_state: Board, player_to_generate: Color) -> MCTSTree {
        let root = MCTSNode::new(initial_state, player_to_generate, None);
        let mut arena: Arena<MCTSNode> = Arena::new();
        let root_index = arena.insert(root);
        MCTSTree { root_index, arena }
    }
}

impl MCTSTree {
    // Creates a new node in this MCTSTree from the given parameters and returns its Index.
    // If a node of these parameters already exists, returns its Index
    fn node(&mut self, state: Board, played_last_move: Color, last_move: Option<Move>) -> Index {
        let new_node = MCTSNode::new(state, played_last_move, last_move);
        for (index, node) in &self.arena {
            if *node == new_node {
                return index;
            }
        }

        self.arena.insert(new_node)
    }
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

impl MCTSTree {
    fn root(&self) -> &MCTSNode {
        self.arena.get(self.root_index).unwrap()
    }
}

/*******************************************************\
|****************      TREE SEARCH      ****************|
\*******************************************************/

impl MCTSTree {
    fn selection(&self) -> Index {
        let mut best_node = self.root();
        let mut best_index = self.root_index;
        let mut best_score = 0.0;
        while best_node.children.len() > 0 {
            let mut best_child_index = best_index;
            for child_idx in &best_node.children {
                // todo: not sure if below is fine to let panic
                let child = self.arena.get(*child_idx).unwrap();
                let child_score = child.uct_score(best_node);
                if child_score > best_score {
                    best_child_index = *child_idx;
                    best_score = child_score;
                }
            }
            best_index = best_child_index;
            // todo: same as above
            best_node = self.arena.get(best_child_index).unwrap();
        }
        best_index
    }
    
    fn expansion(&mut self, node_index: Index) {
        todo!()
    }
    
    fn simulation(&mut self, node_index: Index) {
        todo!()
    }
    
    fn backpropagation(&mut self, node_index: Index) {
        todo!()
    }
}

/*********************************************************\
|************   MOVE GENERATION HEURISTICS   *************|
\*********************************************************/

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
/********************************************************\
|****************     PUBLIC METHODS     ****************|
\********************************************************/

// Generates a move using this Go Engine (MCTS) to play on the given Board
pub(crate) fn generate_move(position: Board, color: Color, iterations: u16) -> Move {
    let mut tree = MCTSTree::new(position, color);

    for _ in 0..iterations {
        // todo: implement below methods and use them appropriately
        let node_index = tree.selection();
        tree.expansion(node_index);
        tree.simulation(node_index);
        tree.backpropagation(node_index);
    }

    // todo: maybe should add helper?
    if tree.root().children.len() == 0 {
        Move::PASS
    } else {
        // todo: find child of max ending value and return its last move
        todo!()
    }
}
