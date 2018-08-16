mod linked_list;
mod solver;
mod stage;

fn main() {
    let stage = stage::Stage::beginner();

    print!("Initial stage:");
    stage.print();

    match solver::solve(stage) {
        Some((moves, final_stage)) => {
            println!("Solution found:");

            // Show the moves necessary to reach the winning stage
            for move_ in moves {
                println!("{}", move_);
            }

            print!("Final stage:");
            final_stage.print();
        }
        None => {
            println!("No solution found")
        }
    }

}
