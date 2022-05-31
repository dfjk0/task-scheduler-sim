use scheduler::*;
use std::io;

const NAME: [&'static str; 10] = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];

fn random(min: u32, max: u32) -> u32 {
    assert!(max > min, "random: invalid operation (max <= min)");
    min + (rand::random::<u32>() % (max - min))
}

fn gen_tasks(num_of_queue: u32) -> Vec<Task> {
    let mut tasks = Vec::new();
    for i in 0..random(3, 10) as usize {
        tasks.push(Task::new(NAME[i], random(0, 10), random(1, 10), random(0, num_of_queue)));
    }
    println!("Tasks was generated.");
    tasks
}

struct Input {
    buffer: String,
    stdin: io::Stdin,
}

use std::io::Write;
impl Input {
    fn new() -> Self {
        Self {
            buffer: String::new(),
            stdin: io::stdin()
        }
    }

    fn get_number(&mut self) -> Option<u32> {
        print!("> ");
        io::stdout().flush().unwrap();
        self.buffer.clear();
        self.stdin.read_line(&mut self.buffer).unwrap();
        self.buffer.trim().parse::<u32>().ok() 
    }

    fn get_char(&mut self) -> Option<char> {
        print!("> ");
        io::stdout().flush().unwrap();
        self.buffer.clear();
        self.stdin.read_line(&mut self.buffer).unwrap();
        self.buffer.trim().chars().nth(0)
    }
}

fn number_of_queue(input: &mut Input) -> u32 {
    println!("Number of Queue (1~10):");
    loop {
        if let Some(n) = input.get_number() {
            if 1 <= n && n <= 10 {
                break n;
            }
        }
        ErrorMsg::invalid_value();
    }
}

fn round_robin_input(input: &mut Input) -> Algorithm {
    println!("RoundRobin: Time Quantum (1~10):");
    let time_quantum = loop {
        if let Some(n) = input.get_number() {
            if 1 <= n && n <= 10 {
                break n;
            }
        }
        ErrorMsg::invalid_value();
    };

    let feedback = loop {
        println!("RoundRobin: Multilevel Feedback? [y/n]");
        break match input.get_char() {
            Some('y') => true,
            Some('n') => false,
            _ => {
                ErrorMsg::invalid_value();
                continue;
            },
        }
    };
    Algorithm::RoundRobin(time_quantum, feedback)
}

fn algorithms(number_of_queue: u32, input: &mut Input) -> Vec<Algorithm> {
    let mut algo = Vec::new();
    for i in 0..number_of_queue {
        loop {
            println!("Queue {} Algorithm (1~3):", i);
            println!("1: ArrivalOrder, 2: ProcessingTimeOrder, 3: RoundRobin");
            match input.get_number() {
                Some(1) => {
                    algo.push(Algorithm::ArrivalOrder);
                    break;
                },
                Some(2) => {
                    algo.push(Algorithm::ProcessingTimeOrder);
                    break;
                },
                Some(3) => {
                    algo.push(round_robin_input(input));
                    break;
                },
                _ => ErrorMsg::invalid_value(),
            }
        }
    }
    algo
}

enum UserSelect {
    ConfigQueue,
    GenTasks,
    RunSim,
    Exit,
}

fn main_menu(input: &mut Input) -> UserSelect {
    loop {
        println!("What do you want to do?");
        println!("1: Create queue list.");
        println!("2: Generate tasks.");
        println!("3: Run simulation.");
        println!("4: Exit");
        match input.get_number() {
            Some(1) => return UserSelect::ConfigQueue,
            Some(2) => return UserSelect::GenTasks,
            Some(3) => return UserSelect::RunSim,
            Some(4) => return UserSelect::Exit,
            _ => ErrorMsg::invalid_value(),

        }
    }
}

fn config_queue(input: &mut Input) -> QueueList {
    let number_of_queue = number_of_queue(input);
    let algorithms = algorithms(number_of_queue, input);
    create_queue_list(algorithms)
}

struct ErrorMsg {}

impl ErrorMsg {
    fn invalid_value() {
        println!("[Error] Invalid value. Try again.");
    }

    fn queue_list_not_exist() {
        println!("[Error] Queue list is not exist. Create a queue list first.");
    }

    fn tasks_not_exist() {
        println!("[Error] Tasks is not exist. Generate tasks first.");
    }
}

fn validation(queue_list: &Option<QueueList>, tasks: &Option<Vec<Task>>) -> Option<(QueueList, Vec<Task>)> {
    if queue_list.is_none() {
        ErrorMsg::queue_list_not_exist();
        return None;
    }

    if tasks.is_none() {
        ErrorMsg::tasks_not_exist();
        return None;
    }

    Some((queue_list.clone().take().unwrap(), tasks.clone().take().unwrap()))
}

fn main() {
    let mut input = Input::new();
    let mut tasks = None;
    let mut queue_list = None;

    loop {
        match main_menu(&mut input) {
            UserSelect::ConfigQueue => {
                queue_list = Some(config_queue(&mut input));
            },
            UserSelect::GenTasks => {
                if let Some(queue_list) = &queue_list {
                    tasks = Some(gen_tasks(queue_list.len() as u32));
                    print_info(tasks.as_ref().unwrap());
                } else {
                    ErrorMsg::queue_list_not_exist();
                }
            },
            UserSelect::RunSim => {
                if let Some((queue_list, tasks)) = validation(&queue_list, &tasks) {
                    let results = run_simulator(queue_list, tasks);
                    print_result(&results);
                    pause();
                }
            }
            UserSelect::Exit => {
                println!("See you next time :)");
                break;
            }
        };
    }
}

