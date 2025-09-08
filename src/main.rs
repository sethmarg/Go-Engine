// Allows the program to run lol
fn main() {
    use std::env;
    use engine::start_go_agent;
    
    let arguments: Vec<String> = env::args().collect();
    start_go_agent(arguments);
}