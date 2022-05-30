use scheduler::*;

const NAME: [&'static str; 10] = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];

fn random(min: u32, max: u32) -> u32 {
    assert!(max > min, "random: invalid operation (max <= min)");
    min + (rand::random::<u32>() % (max - min))
}

fn gen_tasks(num_of_queue: u32) -> Vec<Task> {
    let mut tasks = Vec::new();
    for i in 0..random(3, 10) as usize {
        tasks.push(Task::new(NAME[i], random(0, 10), random(1, 10), random(0, num_of_queue + 1)));
    }
    tasks
}

fn main() {
    let algorithms = vec![Algorithm::ArrivalOrder];
    let queues = create_queue_list(algorithms);
    let tasks = gen_tasks(1);
    print_info(&tasks);
    let results = run_simulator(queues, tasks);
    print_result(&results);
}

