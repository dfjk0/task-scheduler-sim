#![allow(dead_code)]
use std::collections::VecDeque;

fn print_separator() {
    println!("----------------------------------");
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, Default, Clone)]
struct Task {
    name: String,
    arrival_time: u32,
    processing_time: u32,
    finish_time: u32,
    state: State,
}

struct Scheduler {
    time: u32,
    tasks_list: Vec<Task>,
    queue: VecDeque<Task>,
    finished_tasks: Vec<Task>
}

impl Scheduler {
    fn new(tasks_list: Vec<Task>) -> Scheduler {
        Scheduler {
            time: 0,
            tasks_list,
            queue: VecDeque::new(),
            finished_tasks: Vec::new(),
        }
    }

    fn fetch_task(&mut self) -> Vec<Task> {
        let Scheduler {tasks_list, time, ..} = self;

        let mut new_tasks = Vec::new();
        for task in tasks_list.iter_mut() {
            if task.arrival_time <= *time && task.state == State::Watiting {
                new_tasks.push(task.clone());
                task.state = State::Executable;
            }
        }
        new_tasks
    }

    fn check_finish(&self) -> bool {
        self.tasks_list.len() == self.finished_tasks.len()
    }

    fn arrival_order(&mut self) {
        while self.check_finish() {
            let mut new_tasks = self.fetch_task();
            if new_tasks.is_empty() {
                self.time += 1;
                continue;
            }

            new_tasks.sort_by(|a, b| a.arrival_time.cmp(&b.arrival_time));
            for task in new_tasks {
                println!("Fetch Task {}", task.name);
                self.queue.push_back(task);
            }

            let mut task = self.queue.pop_front().unwrap();
            println!("Dispatch Task {}", task.name);
            self.time += task.processing_time;
            task.finish_time = self.time;
            self.finished_tasks.push(task);
        }
    }

    fn print_result(&self) {
        let mut sum = 0;
        for task in self.finished_tasks.iter() {
            print_separator();
            println!("Task {}", task.name);
            println!("    Finish Time       : {}", task.finish_time);
            let turnaround_time = task.finish_time - task.arrival_time;
            println!("    Turnaround Time   : {}", turnaround_time);
            sum += turnaround_time;
        }
        print_separator();
        println!("Average of Turnaround Time: {}", sum as f32 / self.tasks_list.len() as f32);
    }
}

fn task_queue_test_case() -> Vec<Task> {
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
    let tasks = task_queue_test_case();
    let mut scheduler = Scheduler::new(tasks);
    scheduler.arrival_order();
    scheduler.print_result();
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
