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
pub struct Task {
    name: &'static str,
    arrival_time: u32,
    cost: u32,
    processed_time: u32,
    finish_time: u32,
    priority: u32,
    state: State,
}

impl Task {
    pub const fn new(name: &'static str, arrival_time: u32, cost: u32, priority: u32) -> Task {
        Task {
            name,
            arrival_time,
            cost,
            processed_time: 0,
            finish_time: 0,
            priority,
            state: State::Watiting,
        }
    }
}

// Algorithm 
#[derive(Clone)]
pub enum Algorithm {
    ArrivalOrder,
    ProcessingTimeOrder,
    RoundRobin(u32, bool)
}

#[derive(Debug, PartialEq)]
pub struct TaskResult {
    name: &'static str,
    arrive: u32,
    cost: u32,
    priority: u32,
    finish: u32,
    turnaround: u32
}

impl TaskResult {
    pub const fn new(Task {name, arrival_time, cost, priority, ..}: Task, finish: u32) -> TaskResult {
        TaskResult {
            name,
            arrive: arrival_time,
            cost,
            priority,
            finish,
            turnaround: finish - arrival_time,
        }
    }
}

type QueueList = Vec<(VecDeque<Task>, Algorithm)>;

fn print_queue_list(queue_list: &QueueList) {
    use std::io::{stdout, Write};
    for (i, queue) in queue_list.iter().enumerate() {
        print!("Task Queue {}: [ ", i);
        for task in queue.0.iter() {
            print!("{} ", task.name);
        }
        print!("]\n");
    }
    stdout().flush().unwrap();
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
    if new_tasks.len() > 0 { Some(new_tasks) } else { None }
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
        task.processed_time += 1;
        if task.cost <= task.processed_time {
            println!("    Task {} was finished.", task.name);
            task.finish_time = time + 1;
            return Some((task, true));
        } else {
            return Some((task, false));
        }
    }
    None
}

fn add_task(queue_list: &mut QueueList, task: Task) {
    let (queue, algorithm) = &mut queue_list[task.priority as usize];
    queue.push_back(task);
    match algorithm {
        Algorithm::ArrivalOrder => (),
        Algorithm::ProcessingTimeOrder => {
            for i in (2..queue.len()).into_iter().rev() {
                if queue[i].cost < queue[i - 1].cost {
                    queue.swap(i, i - 1);
                }
            }
        },
        Algorithm::RoundRobin(..) => (),
    }
}

fn add_result(result_list: &mut Vec<TaskResult>, time: u32, task: Task) {
    let finish_time = time + 1;
    result_list.push(TaskResult::new(task, finish_time));
}

pub fn dispatch_task(queue_list: &mut QueueList, time: u32, result_list: &mut Vec<TaskResult>, counter: &mut u32) {
    for i in 0..queue_list.len() {
        let (queue, algorithm) = &mut queue_list[i];
        match algorithm {
            Algorithm::ArrivalOrder => {
                match dispatch(queue, time) {
                    Some((task, true)) => add_result(result_list, time, task),
                    Some((task, false)) => queue.push_front(task),
                    None => (),
                }
            },
            Algorithm::ProcessingTimeOrder => {
                match dispatch(queue, time) {
                    Some((task, true)) => add_result(result_list, time, task),
                    Some((task, false)) => queue.push_front(task),
                    None => (),
                }
            },
            Algorithm::RoundRobin(time_quantum, _) => {
                match dispatch(queue, time) {
                    Some((task, true)) => {
                        add_result(result_list, time, task);
                        *counter = 0;
                    },
                    Some((mut task, false)) => {
                        *counter += 1;
                        if counter >= time_quantum {
                            println!("    Timeout Task {}", task.name);
                            task.state = State::Executable;
                            queue.push_back(task);
                            *counter = 0;
                        } else {
                            queue.push_front(task);
                        }
                    },
                    None => (),
                }
            }
        }
    }
}

//validation
fn validation(task_list: &Vec<Task>, queue_list: &QueueList) {
    let max_priority = queue_list.len() as u32;
    for task in task_list.iter() {
        if task.priority >= max_priority {
            panic!("Validation failed")
        }
    }
}

pub fn create_queue_list(algorithm_list: Vec<Algorithm>) -> QueueList {
    let mut queue_list = Vec::with_capacity(algorithm_list.len());
    for algorithm in algorithm_list {
        queue_list.push((VecDeque::new(), algorithm));
    }
    queue_list
}

pub fn run_simulator(mut queue_list: QueueList, mut task_list: Vec<Task>) -> Vec<TaskResult> {
    validation(&task_list, &queue_list);
    println!("\n-- Start Simulator ------------------------");

    task_list.sort_by(|a, b| a.arrival_time.cmp(&b.arrival_time));
    task_list.reverse();
    
    let mut result_list = Vec::new();
    let mut time = 0;
    let mut counter = 0;
    let task_list_len = task_list.len();

    while task_list_len > result_list.len() {
        println!("Time {}-{}", time, time + 1);
        if let Some(tasks) = fetch_new_tasks(&mut task_list, time) {
            for task in tasks {
                println!("    Task {} arrived on Queue {}.", task.name, task.priority);
                add_task(&mut queue_list, task);
            }
        }

        dispatch_task(&mut queue_list, time, &mut result_list, &mut counter);
        print_queue_list(&queue_list);
        print_separator();
        time += 1;
    }

    result_list
}

pub fn print_info(tasks: &Vec<Task>) {
    println!("\n-- Task Informations ----------------------");
    println!("name arrive cost priority");
    for task in tasks.iter() {
        println!("{:>4} {:>6} {:>4} {:>8}",task.name, task.arrival_time, task.cost, task.priority);
    }
}

pub fn print_result(finished_tasks: &Vec<TaskResult>) {
    println!("\n-- Result ---------------------------------");
    let mut sum = 0;
    println!("name arrive cost priority finish turnaround");
    for task in finished_tasks.iter() {
        println!("{:>4} {:>6} {:>4} {:>8} {:>6} {:>10}", 
                 task.name, task.arrive, task.cost,
                 task.priority, task.finish, task.turnaround);
        sum += task.turnaround;
    }
    println!("\nAverage of Turnaround Time: {}", sum as f32 / finished_tasks.len() as f32);
}
