mod scanner;
mod token;

struct Lox {
    had_error: bool,
}
impl Lox {
    fn main(&mut self, args: Vec<String>) {
        if args.len() > 1 {
            println!("Usage: jlox [script]");
            std::process::exit(64);
        } else if args.len() == 1 {
            self.run_file(&args[0]);
        } else {
            self.run_prompt();
        }
    }
    fn run_file(&mut self, path: &String) {
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
            print!("> ");
            let mut line = String::from("");
            std::io::stdin().read_line(&mut line);
            if !line.is_empty() {
                break;
            }
            self.run(line);
            self.had_error = false;
        }
    }
    fn error(&mut self, line: u32, message: String) {
        self.report(line, String::from(""), message);
    }

    fn report(&mut self, line: u32, location: String, message: String) {
        eprintln!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }

    fn run(&mut self, source: String) {
        let scanner = scanner::Scanner::new(&source, |line, message| self.error(line, message));
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{}", token);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
