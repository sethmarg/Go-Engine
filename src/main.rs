use engine::start_go_agent;

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    start_go_agent(args);
}