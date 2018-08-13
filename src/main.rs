mod stage;

use std::collections::{HashSet, VecDeque};

fn main() {
    let stage = stage::Stage::beginner();
    stage.print();

    // Basic path finding algorithm, where nodes in the graph are stages and
    // edges are moves leading from one stage to another
    let mut seen_states = HashSet::new();
    let mut state_queue = VecDeque::new();
    state_queue.push_back((Vec::new(), stage));

    let mut move_buffer = Vec::new();
    while let Some((path, state)) = state_queue.pop_front() {
        // Skip states that we have already seen
        if seen_states.contains(&state) {
            continue
        }

        // Stop when we reach a final state
        if state.game_finished() {
            println!("Solution found");
            for move_ in &path {
                println!("{}", move_);
            }
            println!("Final stage: ");
            state.print();
            return
        }

        // Record that we have seen this state
        seen_states.insert(state.clone());

        // For each possible move, generate its corresponding stage and enqueue it
        move_buffer.clear();
        state.available_moves(&mut move_buffer);
        for &move_ in &move_buffer {
            // Unwrap is OK because we know the move is valid
            let new_state = state.make_move(move_).unwrap();

            // Add the current move to the path
            let mut new_path = path.clone();
            new_path.push(move_);

            state_queue.push_back((new_path, new_state));
        }
    }

    // This part is only reached when no solution was found
    println!("No solution found")
}
