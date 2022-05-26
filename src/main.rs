#![allow(dead_code)]
use std::collections::VecDeque;

fn print_separator() {
    println!("---------------------------------------");
}

#[derive(Debug, PartialEq)]
enum State {
    Execution,
    Watiting,
    Executable
}

impl Default for State {
    fn default() -> State {
        State::Watiting
    }
}

#[derive(Debug, Default, PartialEq)]
struct Task {
    name: String,
    arrival_time: u32,
    processing_time: u32,
    finish_time: u32,
    state: State,
}

fn fetch_new_tasks(task_list: &mut Vec<Task>, time: u32) -> Option<Vec<Task>> {
    if task_list.is_empty() {
        return None;
    }
    let mut new_tasks = Vec::new();
    loop {
        if let Some(task) = task_list.last() {
            if task.arrival_time <= time {
                let mut task = task_list.pop().unwrap();
                task.state = State::Executable;
                new_tasks.push(task);
                continue;
            }
        }
        break;
    }
    if new_tasks.len() > 0 {
        Some(new_tasks)
    } else {
        None
    }
}

fn arrival_order(task_list: &mut Vec<Task>) -> Vec<Task> {
    task_list.sort_by(|a, b| b.arrival_time.cmp(&a.arrival_time));
    let mut time = 0;
    let mut task_queue = VecDeque::new();
    let mut finished_tasks = Vec::new();

    let num_of_tasks = task_list.len();
    while num_of_tasks > finished_tasks.len() {
        println!("Time {}-{}", time, time + 1);
        if let Some(tasks) = fetch_new_tasks(task_list, time) {
            for task in tasks {
                println!("    Task {} arrived.", task.name);
                task_queue.push_back(task);
            }
        }
        match dispatch(&mut task_queue, time) {
            Some((task, true)) => finished_tasks.push(task),
            Some((task, false)) => task_queue.push_front(task),
            None => (),
        }

        print_task_queue(&task_queue);
        print_separator();

        time += 1;
    }
    finished_tasks
}

fn processing_time_order(tasks: &mut Vec<Task>) -> Vec<Task> {
    tasks.sort_by(|a, b| b.arrival_time.cmp(&a.arrival_time));
    let mut time = 0;
    let mut task_queue = VecDeque::new();
    let mut finished_tasks = Vec::new();

    let update_queue = |task_queue: &mut VecDeque<Task> | {
        for i in (2..task_queue.len()).into_iter().rev() {
            if task_queue[i].processing_time < task_queue[i-1].processing_time {
                task_queue.swap(i, i - 1);
            }        
        }
    };

    let num_of_tasks = tasks.len();
    while num_of_tasks > finished_tasks.len() {
        println!("Time {}-{}", time, time + 1);
        if let Some(tasks) = fetch_new_tasks(tasks, time) {
            for task in tasks {
                println!("    Task {} arrived.", task.name);
                task_queue.push_back(task);
                update_queue(&mut task_queue);
            }
        }

        match dispatch(&mut task_queue, time) {
            Some((task, true)) => finished_tasks.push(task),
            Some((task, false)) => task_queue.push_front(task),
            None => ()
        }

        print_task_queue(&task_queue);
        print_separator();

        time += 1;
    }
    finished_tasks
}

fn round_robin(tasks: &mut Vec<Task>, time_quantum: u32) -> Vec<Task> {
    tasks.sort_by(|a, b| b.arrival_time.cmp(&a.arrival_time));
    let mut time = 0;
    let mut counter = 0;
    let mut task_queue = VecDeque::new();
    let mut finished_tasks = Vec::new();

    let num_of_tasks = tasks.len();
    while num_of_tasks > finished_tasks.len() {
        println!("Time {}-{}", time, time + 1);
        if let Some(tasks) = fetch_new_tasks(tasks, time) {
            for task in tasks {
                println!("    Task {} arrived.", task.name);
                task_queue.push_back(task);
            }
        }

        match dispatch(&mut task_queue, time) {
            Some((task, true)) => {
                finished_tasks.push(task);
                counter = 0;
            },
            Some((mut task, false)) => {
                counter += 1;
                if counter >= time_quantum {
                    println!("    Timeout Task {}", task.name);
                    task.state = State::Executable;
                    task_queue.push_back(task);
                    counter = 0;
                } else {
                    task_queue.push_front(task);
                }
            },
            None => (),
        }

        print_task_queue(&task_queue);
        print_separator();

        time += 1;
    }
    finished_tasks
}

fn print_task_queue(task_queue: &VecDeque<Task>) {
    use std::io::{stdout, Write};
    print!("Task Queue: ");
    for task in task_queue.iter() {
        print!("< {} ", task.name);
    }
    print!("\n"); stdout().flush().unwrap();
}

// Finished Some(Task, true)
// NotFinished Some(Task, false)
// Queue is empty None
fn dispatch(task_queue: &mut VecDeque<Task>, time: u32) -> Option<(Task, bool)> {
    if let Some(mut task) = task_queue.pop_front() {
        if task.state == State::Execution {
            println!("    Task {} was executed.", task.name);
        } else {
            println!("    Task {} was dispatched and executed.", task.name);
            task.state = State::Execution;
        }
        task.processing_time -= 1;
        if task.processing_time <= 0 {
            println!("    Task {} was finished.", task.name);
            task.finish_time = time + 1;
            return Some((task, true));
        } else {
            return Some((task, false));
        }
    }
    None
}

fn print_tasks_info(task_list: &Vec<Task>) {
    println!("\n-- Task Informations ----------------------");
    for task in task_list.iter() {
        println!("Task {}:", task.name);
        println!("    Arrival Time      : {}", task.arrival_time);
        println!("    Processing Time   : {}", task.processing_time);
        print_separator();
    }
}

fn print_result(finished_tasks: &Vec<Task>) {
    println!("\n-- Result ---------------------------------");
    let mut sum = 0;
    for task in finished_tasks.iter() {
        println!("Task {}:", task.name);
        println!("    Finish Time       : {}", task.finish_time);
        let turnaround_time = task.finish_time - task.arrival_time;
        println!("    Turnaround Time   : {}", turnaround_time);
        sum += turnaround_time;
        print_separator();
    }
    println!("Average of Turnaround Time: {}", sum as f32 / finished_tasks.len() as f32);
    print_separator();
}

fn test_case() -> Vec<Task> {
    let mut tasks = Vec::new();
    tasks.push(Task {
        name: String::from("A"),
        arrival_time: 0,
        processing_time: 4,
        ..Default::default()
    });
    tasks.push(Task {
        name: String::from("B"),
        arrival_time: 1,
        processing_time: 8,
        ..Default::default()
    });
    tasks.push(Task {
        name: String::from("D"),
        arrival_time: 3,
        processing_time: 2,
        ..Default::default()
    });
    tasks.push(Task {
        name: String::from("C"),
        arrival_time: 5,
        processing_time: 6,
        ..Default::default()
    });
    tasks
}

fn test_case2() -> Vec<Task> {
    let mut tasks = Vec::new();
    tasks.push(Task {
        name: String::from("A"),
        arrival_time: 1,
        processing_time: 4,
        ..Default::default()
    });
    tasks.push(Task {
        name: String::from("B"),
        arrival_time: 1,
        processing_time: 8,
        ..Default::default()
    });
    tasks.push(Task {
        name: String::from("D"),
        arrival_time: 3,
        processing_time: 2,
        ..Default::default()
    });
    tasks.push(Task {
        name: String::from("C"),
        arrival_time: 5,
        processing_time: 6,
        ..Default::default()
    });
    tasks.push(Task {
        name: String::from("E"),
        arrival_time: 5,
        processing_time: 6,
        ..Default::default()
    });
    tasks
}

fn main() {
    let mut tasks = test_case();
    print_tasks_info(&tasks);
    println!("\n-- Start ----------------------------------");
    let finished_tasks = arrival_order(&mut tasks);
    //let finished_tasks = processing_time_order(&mut tasks);
    //let finished_tasks = round_robin(&mut tasks, 2);
    print_result(&finished_tasks);
}
