use scheduler::*;
//priority
#[test]
fn round_robin_priority_test() {
    let algoritms = vec![Algorithm::RoundRobin(2, false); 2];
    run_test(algoritms, test_case, round_robin_priority);
}

#[test]
// Multilevel Feedback Queue
fn mlfq_test() {
    let algoritms = vec![Algorithm::RoundRobin(2, true); 2];
    run_test(algoritms, test_case, mlfq);
}

const TASK_A: Task = Task::new("A", 0, 4, 0);
const TASK_B: Task = Task::new("B", 1, 8, 1);
const TASK_C: Task = Task::new("C", 5, 6, 1);
const TASK_D: Task = Task::new("D", 3, 2, 0);

fn run_test(algorithm_list: Vec<Algorithm>, tasks: fn()->Vec<Task>, expects: fn()->Vec<TaskResult>) {
    let queue_list = create_queue_list(algorithm_list);
    let results = run_simulator(queue_list, tasks());
    assert_eq!(results, expects());
}

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
//    D      3    2        0      6          6
//    C      5    6        0     12         18
//    B      1    8        0     20         20
fn round_robin_priority() -> Vec<TaskResult> {
    let mut results = Vec::new();
    results.push(TaskResult::new(TASK_A,  4));
    results.push(TaskResult::new(TASK_D,  6));
    results.push(TaskResult::new(TASK_C, 18));
    results.push(TaskResult::new(TASK_B, 20));
    results
}

// name arrive cost priority finish turnaround
//    D      3    2        0      5          2
//    A      0    4        0      8          8
//    C      5    6        0     18         13
//    B      1    8        0     20         19
fn mlfq() -> Vec<TaskResult> {
    let mut results = Vec::new();
    results.push(TaskResult::new(TASK_D,  5));
    results.push(TaskResult::new(TASK_A,  8));
    results.push(TaskResult::new(TASK_C, 18));
    results.push(TaskResult::new(TASK_B, 20));
    results
}
