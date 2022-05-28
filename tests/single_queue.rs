use scheduler::*;

#[test] // Ok
fn arrival_order_test() { 
    let algorithm_list = vec![Algorithm::ArrivalOrder];
    run_test(algorithm_list, test_case, arrival_order);
}

#[test] // Ok
fn processed_time_order_test() {
    let algorithm_list = vec![Algorithm::ProcessingTimeOrder];
    run_test(algorithm_list, test_case, processing_time_order);
}

#[test] // Todo
fn round_robin_test() {
    let algorithm_list = vec![Algorithm::RoundRobin(2, false)];
    run_test(algorithm_list, test_case, round_robin);
}

fn run_test(algorithm_list: Vec<Algorithm>, tasks: fn()->Vec<Task>, expects: fn()->Vec<TaskResult>) {
    let queue_list = create_queue_list(algorithm_list);
    let results = run_simulator(queue_list, tasks());
    assert_eq!(results, expects());
}

const TASK_A: Task = Task::new("A",  0, 4, 0);
const TASK_B: Task = Task::new("B",  1, 8, 0);
const TASK_C: Task = Task::new("C",  5, 6, 0);
const TASK_D: Task = Task::new("D",  3, 2, 0);

fn test_case() -> Vec<Task> {
    let mut tasks = Vec::new();
    tasks.push(TASK_A);
    tasks.push(TASK_B);
    tasks.push(TASK_C);
    tasks.push(TASK_D);
    tasks
}

// name arrive cost priority finish turnaround
//    A      0    4        0      4          4
//    B      1    8        0     12         11
//    D      3    2        0     14         11
//    C      5    6        0     20         15
fn arrival_order() -> Vec<TaskResult> {
    let mut results = Vec::new();
    results.push(TaskResult::new(TASK_A,  4));
    results.push(TaskResult::new(TASK_B, 12));
    results.push(TaskResult::new(TASK_D, 14));
    results.push(TaskResult::new(TASK_C, 20));
    results
}

// name arrive cost priority finish turnaround
//    A      0    4        0      4          4
//    B      1    8        0     20         19
//    D      3    2        0      6          3
//    C      5    6        0     12          7
fn processing_time_order() -> Vec<TaskResult> {
    let mut results = Vec::new();
    results.push(TaskResult::new(TASK_A,  4));
    results.push(TaskResult::new(TASK_D,  6));
    results.push(TaskResult::new(TASK_C, 12));
    results.push(TaskResult::new(TASK_B, 20));
    results
}


// name arrive cost priority finish turnaround
//    A      0    4        0      6          6
//    D      3    2        0      8          5
//    B      1    8        0     18         17
//    C      5    6        0     20         15
fn round_robin() -> Vec<TaskResult> {
    let mut results = Vec::new();
    results.push(TaskResult::new(TASK_A,  6));
    results.push(TaskResult::new(TASK_D,  8));
    results.push(TaskResult::new(TASK_B, 18));
    results.push(TaskResult::new(TASK_C, 20));
    results
}
