use std::collections::{HashSet, VecDeque};

use linked_list::List;
use stage::{Stage, Move};

/// Return a tuple containing the moves required to solve the puzzle and the
/// resulting final stage
///
/// If the stage cannot be solved, this function will return `None`
///
/// # Implementation details
///
/// We compute the shortest path using breadth first search. In the graph,
/// stages are nodes and moves are edges.
pub fn solve(stage: Stage) -> Option<(impl Iterator<Item = Move>, Stage)> {
    // Since the graph is cyclic, we need to keep track of nodes we have already seen
    let mut seen_nodes = HashSet::new();
    let mut node_queue = VecDeque::new();
    node_queue.push_back((List::new(), stage));

    let mut move_buffer = Vec::new();
    while let Some((path, state)) = node_queue.pop_front() {
        // Skip states that we have already seen
        if seen_nodes.contains(&state) {
            continue
        }

        // Stop when we reach a final state
        if state.game_finished() {
            // We need to reverse the path, since it contains the moves in reverse order
            return Some((path.into_iter().rev(), state))
        }

        // Record that we have seen this state
        seen_nodes.insert(state.clone());

        // For each possible move, generate its corresponding stage and enqueue it
        move_buffer.clear();
        state.available_moves(&mut move_buffer);
        for &move_ in &move_buffer {
            // Unwrap is OK because `available_moves` only returns valid moves
            let new_state = state.make_move(move_).unwrap();

            // Add the current move to the path
            let new_path = path.push_front(move_);

            node_queue.push_back((new_path, new_state));
        }
    }

    None
}
