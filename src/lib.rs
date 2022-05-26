use std::collections::VecDeque;
use std::ops::{Index, IndexMut};
use std::fmt;

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
    processing_time: u32,
    finish_time: u32,
    priority: u32,
    state: State,
}

// Algorithm 
pub enum Algorithm {
    ArrivalOrder,
    ProcessingTimeOrder,
    RoundRobin(u32, bool)
}

pub struct QueueList(Vec<(VecDeque<Task>, Algorithm)>);

impl fmt::Display for QueueList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Task Queue: ")?;
        for queue in self.0.iter() {
            for task in queue.0.iter() {
                write!(f, "< {}", task.name)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Index<usize> for QueueList {
    type Output = (VecDeque<Task>, Algorithm);
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl IndexMut<usize> for QueueList {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl QueueList {
    pub fn new(algorithm_list: Vec<Algorithm>) -> QueueList {

        let mut queue_list = Vec::new();
        for algo in algorithm_list {
            queue_list.push((VecDeque::new(), algo));
        }
        QueueList(queue_list)
    }

    pub fn add_task(&mut self, task: Task) {
        let (queue, algorithm) = &mut self.0[task.priority as usize];
        queue.push_back(task);
        match algorithm {
            Algorithm::ArrivalOrder => (),
            _ => todo!("Not Implement"),
        }
    }

    pub fn dispatch_task(&mut self, time: u32, finished_task_list: &mut Vec<Task>) {
        for (queue, algorithm) in self.0.iter_mut() {
            match algorithm {
                Algorithm::ArrivalOrder => {
                    match dispatch(queue, time) {
                        Some((task, true)) => finished_task_list.push(task),
                        Some((task, false)) => queue.push_front(task),
                        None => (),
                    }
                },
                _ => todo!("Not Implement")
           }
        }
    }
}
//validation
fn validation(task_list: &Vec<Task>, query_list: &QueueList) {
    let max_priority = query_list.0.len() as u32;
    for task in task_list.iter() {
        if task.priority >= max_priority {
            panic!("Validation failed")
        }
    }
}

pub fn run_simulator(mut query_list: QueueList, mut task_list: Vec<Task>) -> Vec<Task> {
    validation(&task_list, &query_list);

    task_list.sort_by(|a, b| a.arrival_time.cmp(&b.arrival_time));

    let finished_task_list = Vec::new();
    let mut time = 0;
    let task_list_len = task_list.len();

    while task_list_len > finished_task_list.len() {
        println!("Time {}-{}", time, time + 1);
        if let Some(tasks) = fetch_new_tasks(&mut task_list, time) {
            for task in tasks {
                println!("    Task {} arrived on Queue {}.", task.name, task.priority);
                query_list.add_task(task);
            }
        }

        time += 1;
        println!("{}", query_list);
        print_separator();
    }

    finished_task_list
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
