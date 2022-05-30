use scheduler::*;

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
    tasks
}

use std::io;
struct Input {
    buffer: String,
    stdin: io::Stdin,
}

impl Input {
    fn new() -> Self {
        Self {
            buffer: String::new(),
            stdin: io::stdin()
        }
    }

    fn get_number(&mut self) -> Option<u32> {
        self.buffer.clear();
        self.stdin
            .read_line(&mut self.buffer)
            .expect("failed to read from stdin");
        self.buffer.trim().parse::<u32>().ok() 
    }

    fn get_char(&mut self) -> Option<char> {
        self.buffer.clear();
        self.stdin
            .read_line(&mut self.buffer)
            .expect("failed to read from stdin");
        self.buffer.trim().chars().nth(0)
    }


}

fn pause() {
    use std::io::prelude::*;
    write!(io::stdout(), "Press enter key to continue...").unwrap();
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}

fn number_of_queue(input: &mut Input) -> u32 {
    println!("Number of Queue (1~10):");
    loop {
        if let Some(n) = input.get_number() {
            if 1 <= n && n <= 10 {
                break n;
            }
        }
        println!("Invalid value. Try again.");
    }
}

fn round_robin_input(input: &mut Input) -> Algorithm {
    println!("Time Quantum (1~10):");
    let time_quantum = loop {
        if let Some(n) = input.get_number() {
            if 1 <= n && n <= 10 {
                break n;
            }
        }
        println!("Invalid value. Try again.");
    };

    let feedback = loop {
        println!("Multilevel Feedback? [y/n]");
        break match input.get_char() {
            Some('y') => true,
            Some('n') => false,
            _ => {
                println!("Invalid value. Try again.");
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
                _ => println!("Invalid value. Try again."),
            }
        }
    }
    algo
}

fn main() {
    let mut input = Input::new();
    let number_of_queue = number_of_queue(&mut input);
    let algorithms = algorithms(number_of_queue, &mut input);
    let queues = create_queue_list(algorithms);
    let tasks = gen_tasks(number_of_queue);
    print_info(&tasks);
    pause();
    let results = run_simulator(queues, tasks);
    pause();
    print_result(&results);
}

