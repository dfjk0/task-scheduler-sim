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

    Some(new_tasks)
}

fn arrival_order(task_list: &mut Vec<Task>) -> Vec<Task> {
    task_list.sort_by(|a, b| b.arrival_time.cmp(&a.arrival_time));
    let mut time = 0;
    let mut task_queue = VecDeque::new();
    let mut finished_tasks = Vec::new();

    let num_of_tasks = task_list.len();
    while num_of_tasks > finished_tasks.len() {
        println!("Time {}", time);
        if let Some(tasks) = fetch_new_tasks(task_list, time) {
            for task in tasks {
                println!("    Task {} arrived.", task.name);
                task_queue.push_back(task);
            }
        }
        time += 1;
        dispatch(&mut task_queue, &mut finished_tasks, time);

        print_task_queue(&task_queue);
        print_separator();
    }
    finished_tasks
}

fn processing_time_order(tasks: &mut Vec<Task>) -> Vec<Task> {
    tasks.sort_by(|a, b| b.arrival_time.cmp(&a.arrival_time));
    let mut time = 0;
    let mut task_queue = VecDeque::new();
    let mut finished_tasks = Vec::new();

    let update_queue = |task_queue: &mut VecDeque<Task> | {
        for i in (1..task_queue.len()).into_iter().rev() {
            if task_queue[i].processing_time < task_queue[i-1].processing_time {
                task_queue.swap(i, i - 1);
            }        
        }
    };

    let num_of_tasks = tasks.len();
    while num_of_tasks > finished_tasks.len() {
        println!("Time {}", time);
        if let Some(tasks) = fetch_new_tasks(tasks, time) {
            for task in tasks {
                println!("    Task {} arrived.", task.name);
                task_queue.push_back(task);
                update_queue(&mut task_queue);
            }
        }
        time += 1;
        dispatch(&mut task_queue, &mut finished_tasks, time);

        print_task_queue(&task_queue);
        print_separator();
    }
    finished_tasks
}

fn round_robin(tasks: &mut Vec<Task>, time_quantum: u32) -> Vec<Task> {
    tasks.sort_by(|a, b| b.arrival_time.cmp(&a.arrival_time));
    let mut time = 0;
    let mut task_queue = VecDeque::new();
    let mut finished_tasks = Vec::new();

    let num_of_tasks = tasks.len();
    while num_of_tasks > finished_tasks.len() {
        println!("Time {}", time);
        if let Some(tasks) = fetch_new_tasks(tasks, time) {
            for task in tasks {
                println!("    Task {} arrived.", task.name);
                task_queue.push_back(task);
            }
        }
        time += 1;
        let finished = dispatch(&mut task_queue, &mut finished_tasks, time);
        if time % time_quantum == 0 && !finished {
            if let Some(task) = task_queue.pop_front() {
                println!("Timeout Task {}", task.name);
                task_queue.push_back(task);
            }
        }

        print_task_queue(&task_queue);
        print_separator();
    }
    finished_tasks
}

fn print_task_queue(task_queue: &VecDeque<Task>) {
    use std::io::{stdout, Write};
    print!("Queue: ");
    for task in task_queue.iter() {
        print!("< {} ", task.name);
    }
    print!("\n"); stdout().flush().unwrap();
}

fn dispatch(task_queue: &mut VecDeque<Task>, finished_tasks: &mut Vec<Task>, time: u32) -> bool {
    if let Some(mut task) = task_queue.pop_front() {
        println!("    Task {} was dispatched and executed.", task.name);
        task.processing_time -= 1;
        if task.processing_time <= 0 {
            println!("    Task {} was finished.", task.name);
            task.finish_time = time;
            finished_tasks.push(task);
            return true;
        } else {
            task_queue.push_front(task);
            return false;
        }
    }
    false
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
    let mut task_queue = Vec::new();
    task_queue.push(Task {
        name: String::from("A"),
        arrival_time: 0,
        processing_time: 4,
        ..Default::default()
    });
    task_queue.push(Task {
        name: String::from("B"),
        arrival_time: 1,
        processing_time: 8,
        ..Default::default()
    });
    task_queue.push(Task {
        name: String::from("D"),
        arrival_time: 3,
        processing_time: 2,
        ..Default::default()
    });
    task_queue.push(Task {
        name: String::from("C"),
        arrival_time: 5,
        processing_time: 6,
        ..Default::default()
    });
    task_queue
}

fn task_queue_init() -> VecDeque<Task> {
    let mut input = String::new();
    let mut task_queue = VecDeque::new();
    println!("Number of tasks: ");
    let num_of_tasks = get_integer(&mut input);

    for _ in 0..num_of_tasks {
        print_separator();
        println!("Task Name:");
        get_string(&mut input);
        let name = input.clone();

        println!("Processing Time:");
        let processing_time = get_integer(&mut input);

        println!("Arrival Time:");
        let arrival_time = get_integer(&mut input);

        let task = Task {
            name,
            processing_time,
            arrival_time,
            finish_time: 0,
            state: State::Executable,
        };
        task_queue.push_back(task);
    }
    task_queue
}

fn main() {
    let mut tasks = test_case();
    print_tasks_info(&tasks);
    println!("\n-- Start ----------------------------------");
    //let finished_tasks = arrival_order(&mut tasks);
    //let finished_tasks = processing_time_order(&mut tasks);
    let finished_tasks = round_robin(&mut tasks, 2);
    print_result(&finished_tasks);
}

fn get_integer(input: &mut String) -> u32 {
    input.clear();
    loop {
        get_string(input);
        if let Ok(integer) = input.trim().parse::<u32>() {
            return integer;
        }

        println!("This was not an integer. Try again.");
        input.clear();
    }
}

fn get_string(input: &mut String) {
    input.clear();
    std::io::stdin()
        .read_line(input)
        .expect("failed to read from standard input.");
    input.pop();
}
