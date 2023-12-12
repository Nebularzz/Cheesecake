// apolgy for bad code
// where were u wen nebular write bad code
// i was at house eating dorito when phone ring
// "Nebular write bad code"
// "no"

use std::process::Termination;

#[derive(Debug)]
enum Error {
    FileError,
    InputError,
    EmptyFileError,
    StackOverflow,
}

impl Termination for Error {
    fn report(self) -> std::process::ExitCode {
        match self {
            Self::FileError => {
                std::process::ExitCode::from(1)
            }

            Self::InputError => {
                eprintln!("Please enter the source file!");
                std::process::ExitCode::from(2)
            }

            Self::EmptyFileError => {
                eprintln!("No cheesecake without the program! Please actually enter some code.");
                std::process::ExitCode::from(3)
            }

            Self::StackOverflow => {
                eprintln!("Stack overflow");
                std::process::ExitCode::from(4)
            }
        }
    }
}

struct ExitCode {
    code: u8
}

impl Termination for ExitCode {
    fn report(self) -> std::process::ExitCode {
        std::process::ExitCode::from(self.code)
    }
}


fn main() -> Result<ExitCode, Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(Error::InputError)
    }

    let source = std::fs::read_to_string(&args[1]);

    if source.is_err() {
        eprintln!("Some file error happened: {:?}", source); // I have to keep this here
        return Err(Error::FileError)
    }

    let source: String = source.unwrap();

    if source.is_empty() {
        return Err(Error::EmptyFileError);
    }

    let lines: Vec<String> = lines(source);
    let mut stack = Stack::new();
    execute(parse_lines(lines), &mut stack)
}

fn arithmetic(operator: char, val1: usize, val2: usize) -> usize {
    match operator {
        '+' => {
            return val2 + val1;
        }

        '-' => {
            return val2 - val1;
        }

        '*' => {
            return val2 * val1;
        }

        '/' => {
            return val2 / val1;
        }

        _ => {
            panic!("Invalid operator!");
        }
    }
}

fn execute(instructions: Vec<Instruction>, stack: &mut Stack) -> Result<ExitCode, Error> {
    let mut index: usize = 0;

    while index < instructions.len() {
        let instruction: &Instruction = &instructions[index];

        use Command as C;
        match instruction.command {
            C::Push => {
                stack.push(instruction.argument.unwrap())?
            }

            C::Discard => {
                let result = stack.pop();
                if result.is_err() {
                    return Err(result.err().unwrap());
                }
            }

            C::Duplicate => {
                let val = stack.peek()?;
                stack.push(val)?
            }

            C::Swap => {
                let popped1 = stack.pop()?;
                let popped2 = stack.pop()?;

                stack.push(popped1)?;
                stack.push(popped2)?
            }

            C::Add => {
                let val1: usize = stack.pop()?;
                let val2: usize = stack.pop()?;

                let result = arithmetic('+', val1, val2);
                stack.push(result)?
            }

            C::Sub => {
                let val1: usize = stack.pop()?;
                let val2: usize = stack.pop()?;

                let result = arithmetic('-', val1, val2);
                stack.push(result)?
            }

            C::Mul => {
                let val1: usize = stack.pop()?;
                let val2: usize = stack.pop()?;

                let result = arithmetic('*', val1, val2);
                stack.push(result)?
            }

            C::Div => {
                let val1: usize = stack.pop()?;
                let val2: usize = stack.pop()?;

                let result = arithmetic('/', val1, val2);
                stack.push(result)?
            }

            C::Jiz => {
                let val = stack.pop()?;

                if val == 0 {
                    index = instruction.argument.unwrap();
                    continue;
                }
            }

            C::PrintAscii => {
                let val = stack.pop()?;
                print!("{}", val as u8 as char);
            }

            C::PrintNumber => {
                let val = stack.pop()?;
                print!("{}", val);
            }

            C::Input => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Failed to take input!");
                input = input.trim().to_owned();
                let mut result: Result<(), Error> = Err(Error::EmptyFileError);

                for char in input.chars() {
                    result = stack.push(char as usize);
                }

                result?
            }
        }

        index += 1;
    }

    Ok(ExitCode { code: 0 })
}

fn parse_lines(lines: Vec<String>) -> Vec<Instruction> {
    let mut rt_vec: Vec<Instruction> = Vec::new();

    for line in lines.iter() {
        rt_vec.push(command(line))
    }

    rt_vec
}

fn lines(source: String) -> Vec<String> {
    let collected: Vec<&str> = source.split("\n").collect();
    let mut stringified: Vec<String> = Vec::new();

    for str in collected.iter() {
        stringified.push(str.trim().to_owned().to_owned())
    }

    stringified
}

fn parse_number(num: String) -> usize {
    let mut count: usize = 0;
    if &num[0..4] != "cake" {
        panic!("This number isn't delicious enough!");
    }

    for char in num.chars() {
        if char == 'e' {
            count += 1;
        }
    }

    count -= 1;
    count
}

fn command(line: &str) -> Instruction {
    let split: Vec<&str> = line.split(" cheesecake ").collect();

    match split[0] {
        "cheese" => {
            Instruction::new(Command::Push, Some(parse_number(split[1].to_owned())))
        }

        "cake" => {
            Instruction::new(Command::Discard, None)
        }

        "cheese cake" => {
            Instruction::new(Command::Duplicate, None)
        }

        "cake cheese" => {
            Instruction::new(Command::Swap, None)
        }

        "cheese cheese" => {
            Instruction::new(Command::Add, None)
        }

        "cake cake" => {
            Instruction::new(Command::Sub, None)
        }

        "cheese cheese cheese" => {
            Instruction::new(Command::Mul, None)
        }

        "cheese cheese cake" => {
            Instruction::new(Command::Div, None)
        }

        "cheese cake cake" => {
            Instruction::new(Command::Jiz, Some(parse_number(split[1].to_owned())))
        }

        "cake cake cake" => {
            Instruction::new(Command::PrintAscii, None)
        }

        "cheese cheese cheese cheese" => {
            Instruction::new(Command::PrintNumber, None)
        }

        "cheese cheese cheese cake" => {
            Instruction::new(Command::Input, None)
        }

        _ => {panic!("This is not a valid command! \"{}\"", split[0])}
    }
}

struct Instruction {
    command: Command,
    argument: Option<usize>
}

impl Instruction {
    fn new(command: Command, argument: Option<usize>) -> Instruction {
        Instruction { command, argument }
    }
}

enum Command {
    Push,
    Discard,
    Duplicate,
    Swap,
    Add,
    Sub,
    Mul,
    Div,
    Jiz,
    PrintAscii,
    PrintNumber,
    Input
}


struct Stack {
    stack: [usize; 512],
    sp: usize
}

impl Stack {
    fn push(&mut self, v: usize) -> Result<(), Error> {
        if self.sp >= 512 {
            return Err(Error::StackOverflow);
        }
        self.sp += 1;
        self.stack[self.sp] = v;

        Ok(())
    }

    fn pop(&mut self) -> Result<usize, Error> {
        let val = self.stack[self.sp];
        self.sp -= 1;
        Ok(val)
    }

    fn peek(&mut self) -> Result<usize, Error> {
        Ok(self.stack[self.sp])
    }

    fn new() -> Stack {
        Stack { stack: [0; 512], sp: 0 }
    }
}
