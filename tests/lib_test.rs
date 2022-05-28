#![allow(dead_code)]
use scheduler::*;

#[test]
fn basic_test() {
    assert_eq!(1 + 1, 2);
}

#[test]
fn create_queue_list_test() { 
    let algorithm_list = vec![Algorithm::ArrivalOrder];
    let query_list = create_queue_list(algorithm_list);
    let task_list = test_case1();
    let finished_task_list = run_simulator(query_list, task_list);

    assert_eq!(finished_task_list, test_case_answer1());
}

fn test_case1() -> Vec<Task> {
    let mut tasks = Vec::new();
    // Task::new(name, arrival_time, processing_time, priority)
    tasks.push(Task::new("A", 0, 4, 0));
    tasks.push(Task::new("B", 1, 8, 0));
    tasks.push(Task::new("C", 5, 6, 0));
    tasks.push(Task::new("D", 3, 2, 0));
    tasks
}

fn test_case_answer1() -> Vec<TaskResult> {
    let mut results = Vec::new();
    // TaskResult::new(name, finish_time, turnaround_time)
    results.push(TaskResult::new("A",  4,  4));
    results.push(TaskResult::new("B", 12, 11));
    results.push(TaskResult::new("D", 14, 11));
    results.push(TaskResult::new("C", 20, 15));
    results
}

fn test_case2() -> Vec<Task> {
    let mut tasks = Vec::new();
    tasks.push(Task::new("A", 1, 4, 0));
    tasks.push(Task::new("B", 1, 8, 0));
    tasks.push(Task::new("C", 5, 6, 0));
    tasks.push(Task::new("D", 3, 2, 0));
    tasks.push(Task::new("E", 30, 6, 0));
    tasks
}
