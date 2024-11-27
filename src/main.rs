mod scanner;
mod token;

struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Lox {
        return Lox { had_error: false };
    }
    fn main(&mut self, args: Vec<String>) {
        if args.len() > 2 {
            println!("Usage: rlox [script]");
            std::process::exit(64);
        } else if args.len() == 2 {
            self.run_file(&args[1]);
        } else {
            self.run_prompt();
        }
    }
    fn run_file(&mut self, path: &String) {
        let mut current_dir = std::env::current_dir().unwrap();
        current_dir.push(path);
        println!("Running file: {:?}", current_dir);
        let content = std::fs::read_to_string(path);
        match content {
            Ok(content) => {
                self.run(content);
                if self.had_error {
                    std::process::exit(65);
                }
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                std::process::exit(74);
            }
        }
    }

    fn run_prompt(&mut self) {
        loop {
            print!("> "); // BUG: this doesn't display a prompt as wanted
            let mut line = String::new();
            match std::io::stdin().read_line(&mut line) {
                Ok(_) => {
                    if line.is_empty() {
                        break;
                    }
                    self.run(line)
                }
                Err(error) => eprintln!("Unexpected error: {}", error),
            }
            self.had_error = false;
        }
    }
    fn error_handler(&mut self, line: u32, message: &str) {
        self.report(line, String::from(""), message);
    }

    fn report(&mut self, line: u32, location: String, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }

    fn run(&mut self, source: String) {
        let scanner =
            scanner::Scanner::new(&source, |line, message| self.error_handler(line, message));
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{:#?}", token);
        }
    }
}

fn main() {
    let mut lox = Lox::new();
    let args = std::env::args().collect();
    lox.main(args)
}
