use super::*;
use thunderdome::*;
/******************************************************\
|****************      CONSTANTS       ****************|
\******************************************************/

const RESIGNATION_THRESHOLD: f64 = 60.0;

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
    parent: Option<Index>,
    children: Vec<Index>,
    total_visits: u16,
    winning_visits: u16,
    score: f64,
    simulated: bool
}

/*****************************************************\
|****************        SETUP        ****************|
\*****************************************************/

impl MCTSNode {
    // Creates a new MCTSNode with the given parameters, and setting the others to their default value
    fn new(state: Board, played_last_move: Color) -> MCTSNode {
        MCTSNode {
            state,
            played_last_move,
            parent: None,
            children: vec![],
            total_visits: 0,
            winning_visits: 0,
            score: 0.0,
            simulated: false,
        }
    }
}

impl PartialEq for MCTSNode {
    // Custom equality function that only checks board state fields,
    // does not check any Monte Carlo related fields
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.played_last_move == other.played_last_move
    }
}

impl MCTSTree {
    // Creates a new MCTSTree and
    fn new(initial_state: &Board, player_to_generate: Color) -> MCTSTree {
        let root = MCTSNode::new(initial_state.deepcopy(), player_to_generate.opposite_color());
        let mut arena: Arena<MCTSNode> = Arena::new();
        let root_index = arena.insert(root);
        MCTSTree { root_index, arena }
    }
}

impl MCTSTree {
    // Creates a new node in this MCTSTree from the given parameters and returns its Index.
    // If a node of these parameters already exists, returns its Index
    fn node(&mut self, state: Board, played_last_move: Color) -> Index {
        let new_node = MCTSNode::new(state, played_last_move);
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

        if node_visits == 0.0 {
            return f64::MAX;
        }

        node_wins / node_visits + uct_constant * f64::sqrt(f64::ln(parent_visits) / node_visits)
    }
}

impl MCTSTree {
    fn root(&self) -> &MCTSNode {
        self.arena.get(self.root_index).unwrap()
    }

    fn set_child(&mut self, parent_index: Index, child_index: Index) {
        if self.arena.contains(parent_index) && self.arena.contains(child_index) {
            let parent = self.arena.get_mut(parent_index).unwrap();
            parent.children.push(child_index);
        }
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
        while best_node.children.len() > 0 && !best_node.simulated {
            let mut best_child_index = best_index;
            for child_idx in &best_node.children {
                if self.arena.contains(*child_idx) {
                    let child = self.arena.get(*child_idx).unwrap();
                    let child_score = child.uct_score(best_node);
                    if child_score > best_score {
                        best_child_index = *child_idx;
                        best_score = child_score;
                    }
                } else {
                    panic!("Node index does not exist in the MCTS Tree");
                }
            }
            best_index = best_child_index;
            // this one is definitely safe tho
            best_node = self.arena.get(best_child_index).unwrap();
        }
        best_index
    }

    fn expansion(&mut self, node_index: Index) {
        if !self.arena.contains(node_index) {
            panic!("Node index does not exist in the MCTS Tree");
        }

        let (child_player, candidate_moves, current_state) = {
            let node = self.arena.get_mut(node_index).unwrap();
            node.simulated = false;
            if node.is_game_over() {
                return; // maybe should panic?
            }
            let child_player = node.played_last_move.opposite_color();
            let candidate_moves = node.generate_candidate_moves();
            let current_state = node.state.deepcopy();
            (child_player, candidate_moves, current_state)
        };

        for candidate in candidate_moves {
            let mut child_state = current_state.deepcopy();
            if child_state.play(Move::MOVE(candidate, child_player)) {
                let child_idx = self.node(child_state, child_player);
                self.set_child(node_index, child_idx);
            }
        }
    }

    fn simulation(&mut self, node_index: Index) -> (Index, f64) {
        // todo: placeholder logic, replace with tromp-taylor scoring, forfeit cutoffs to reduce moves played, etc.
        if !self.arena.contains(node_index) {
            panic!("Node index does not exist in the MCTS Tree");
        }

        self.arena.get_mut(node_index).unwrap().simulated = true;
        let (end_index, end_state) = {
            let mut cur_index = node_index;
            for iter in 0..1500 {
                let cur_node = self.arena.get(node_index).unwrap();
                if cur_node.should_resign(RESIGNATION_THRESHOLD) {
                    break;
                }

                let mut cur_state = cur_node.state.deepcopy();
                let player = cur_node.played_last_move.opposite_color();
                let mov = cur_node.generate_playout_move(player);

                if mov == Move::PASS {
                    continue; // kind of want to end playout after two passes but whatever
                } else {
                    if cur_state.play(mov) {
                        let next_node_index = self.node(cur_state, player);
                        self.set_child(cur_index, next_node_index);
                        cur_index = next_node_index;
                    }
                }
            }
            (cur_index, &self.arena.get(cur_index).unwrap().state)
        };

        (end_index, end_state.estimate_score())
    }

    fn backpropagation(&mut self, leaf_index: Index, score: f64) {
        if !self.arena.contains(leaf_index) {
            panic!("Node index does not exist in the MCTS Tree");
        }

        let mut node_index = Some(leaf_index);
        while node_index.is_some() {
            let cur_node = self.arena.get_mut(node_index.unwrap()).unwrap();
            if score > 0.0 && cur_node.played_last_move == Color::BLACK {
                cur_node.winning_visits += 1;
            } else if score < 0.0 && cur_node.played_last_move == Color::WHITE {
                cur_node.winning_visits += 1;
            }
            cur_node.total_visits += 1;

            node_index = cur_node.parent;
        }
    }
}

/*********************************************************\
|************   MOVE GENERATION HEURISTICS   *************|
\*********************************************************/

impl MCTSNode {
    // generates a move to simulate playouts with
    // todo: currently temporary random logic. implement influence maps, move and board scoring, shape moves, etc.
    fn generate_playout_move(&self, color: Color) -> Move {
        use rand::Rng;
        if self.state.size == BoardSize::NINETEEN {
            if let Some(intsc) = self.generate_opening_move() {
                return Move::MOVE(intsc, color);
            }
        }

        let weakest_engine_group = self.state.weakest_group(&color);
        let weakest_opponent_group = self.state.weakest_group(&color.opposite_color());

        // attempt to capture group if possible
        if weakest_opponent_group.len() == 1
            && self.state.can_place_stone_at(&weakest_opponent_group[0])
        {
            return Move::MOVE(weakest_opponent_group[0], color);
        }

        // attempt to save threatened group
        if weakest_engine_group.len() == 1
            && self.state.can_place_stone_at(&weakest_engine_group[0])
        {
            return Move::MOVE(weakest_engine_group[0], color);
        }

        // surround opponent group
        if weakest_opponent_group.len() > 0
            && weakest_opponent_group.len() <= weakest_engine_group.len()
        {
            let rand_idx = rand::thread_rng().gen_range(0..weakest_opponent_group.len());
            return Move::MOVE(weakest_opponent_group[rand_idx], color);
        } else if weakest_engine_group.len() > 0
            && weakest_engine_group.len() <= weakest_opponent_group.len()
        {
            // extend own group
            let rand_idx = rand::thread_rng().gen_range(0..weakest_engine_group.len());
            return Move::MOVE(weakest_engine_group[rand_idx], color);
        }

        // random tenuki
        for offset in 2..0 {
            let random_intsc = self.state.random_intersection(offset);
            if self.state.can_place_stone_at(&random_intsc)
                && self.state.diamond(&random_intsc) == None
            {
                return Move::MOVE(random_intsc, color);
            }
        }

        Move::PASS
    }

    // Generates a move meant to be played in the opening of the game
    // todo: temp moves, and probably shouldn't be chosen randomly
    fn generate_opening_move(&self) -> Option<Intersection> {
        use ColumnIdentifier::*;
        use rand::Rng;
        let fuseki: Vec<Intersection> = vec![
            Intersection::new(D, 4),
            Intersection::new(Q, 4),
            Intersection::new(Q, 16),
            Intersection::new(F, 17),
            Intersection::new(C, 14),
            Intersection::new(F, 3),
            Intersection::new(C, 6),
            Intersection::new(R, 6),
            Intersection::new(O, 3),
            Intersection::new(R, 14),
            Intersection::new(O, 17),
            Intersection::new(C, 10),
            Intersection::new(R, 10),
            Intersection::new(K, 17),
            Intersection::new(K, 3),
            Intersection::new(E, 10),
            Intersection::new(P, 10),
            Intersection::new(K, 15),
            Intersection::new(K, 5),
            Intersection::new(K, 10),
        ];
        let rand_idx = rand::thread_rng().gen_range(0..fuseki.len());
        if self.state.can_place_stone_at(&fuseki[rand_idx]) {
            Some(fuseki[rand_idx])
        } else {
            None
        }
    }

    // Generates candidate moves for the engine to consider playing
    // todo: terrible logic
    fn generate_candidate_moves(&self) -> Vec<Intersection> {
        use ColumnIdentifier::*;
        let mut moves = vec![
            Intersection::new(D, 16),
            Intersection::new(D, 4),
            Intersection::new(Q, 4),
            Intersection::new(Q, 16),
        ];

        for intsc in self.state.weakest_group(&Color::BLACK) {
            moves.push(intsc);
        }

        for intsc in self.state.weakest_group(&Color::WHITE) {
            moves.push(intsc);
        }

        moves.push(self.state.random_intersection(2));

        moves
    }

    // Checks if the game is over by seeing if there are any candidate moves to play
    // todo: terrible logic
    fn is_game_over(&self) -> bool {
        self.generate_candidate_moves().len() == 0
    }

    // should the engine resign in this position
    fn should_resign(&self, resign_threshold: f64) -> bool {
        if self.state.move_number > 100 {
            let to_play = self.played_last_move.opposite_color();
            let score = self.state.estimate_score();

            match to_play {
                Color::BLACK => score < -resign_threshold,
                Color::WHITE => score > resign_threshold,
            }
        } else {
            false
        }
    }
}

/********************************************************\
|****************     PUBLIC METHODS     ****************|
\********************************************************/

// Generates a move using this Go Engine (MCTS) to play on the given Board
pub(crate) fn generate_move(position: &Board, color: Color, iterations: u16) -> Move {
    let mut tree = MCTSTree::new(position, color);
    if tree.root().should_resign(RESIGNATION_THRESHOLD) {
        return Move::RESIGN;
    }

    for iter in 0..iterations {
        // eprintln!("MCTS Iteration {iter}");
        let node_index = tree.selection();
        tree.expansion(node_index);
        let (leaf_index, score) = tree.simulation(node_index);
        tree.backpropagation(leaf_index, score);
    }

    // todo: maybe should add helper?
    let mut best_move = Move::PASS;
    let mut best_visits: u16 = 0;
    for child_idx in &tree.root().children {
        let child = tree.arena.get(*child_idx).unwrap();
        if child.total_visits > best_visits {
            best_visits = child.total_visits;
            best_move = child.state.last_move;
        }
    }

    best_move
}

#[test]
fn test_should_resign() {
    use ColumnIdentifier::*;
    let mut b = Board::new(BoardSize::NINE);

    b.play(Move::MOVE(Intersection::new(C, 7), Color::BLACK));
    b.play(Move::MOVE(Intersection::new(G, 3), Color::WHITE));
    b.play(Move::MOVE(Intersection::new(D, 7), Color::BLACK));
    b.play(Move::MOVE(Intersection::new(G, 2), Color::WHITE));
    b.play(Move::MOVE(Intersection::new(D, 8), Color::BLACK));
    b.play(Move::MOVE(Intersection::new(G, 1), Color::WHITE));
    b.play(Move::MOVE(Intersection::new(D, 9), Color::BLACK));
    b.play(Move::MOVE(Intersection::new(H, 3), Color::WHITE));
    b.play(Move::MOVE(Intersection::new(C, 6), Color::BLACK));
    b.play(Move::MOVE(Intersection::new(J, 3), Color::WHITE));
    b.play(Move::MOVE(Intersection::new(B, 6), Color::BLACK));
    b.play(Move::MOVE(Intersection::new(J, 4), Color::WHITE));
    b.play(Move::MOVE(Intersection::new(A, 6), Color::BLACK));
    b.play(Move::MOVE(Intersection::new(F, 1), Color::WHITE));
    b.move_number = 101;
    
    let mcts_black = MCTSTree::new(&b, Color::BLACK);
    let mcts_white = MCTSTree::new(&b, Color::WHITE);

    assert_eq!(b.estimate_score(), -2.5); // black = 15, white = 11, komi = 6.5
    assert_eq!(mcts_black.root().played_last_move, Color::WHITE);
    assert_eq!(mcts_white.root().played_last_move, Color::BLACK);
    
    assert_eq!(mcts_black.root().should_resign(1.0), true); // black should resign at threshold of 1.0
    assert_eq!(mcts_white.root().should_resign(1.0), false); // white should not resign at threshold of 1.0

    assert_eq!(mcts_black.root().should_resign(5.0), false); // black should not resign at threshold of 5.0
}
