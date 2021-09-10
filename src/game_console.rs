use std::collections::HashSet;

#[derive(Clone)]
enum OpCode {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

pub enum ExitReason {
    Loop,
    Terminate,
}

#[derive(Clone)]
pub struct Program {
    codes: Vec<OpCode>,
}

#[derive(Default)]
pub struct ExecutionContext {
    program_counter: i32,
    accumulator: i32,
    executed_instructions: HashSet<i32>,
}

impl Program {
    pub fn new(s: &str) -> Program {
        Program {
            codes: s.trim().lines().map(OpCode::new).collect(),
        }
    }

    pub fn modify_to_terminate(&self) -> Program {
        for i in 0..self.codes.len() {
            let mut program = self.clone();
            match program.codes[i] {
                OpCode::Acc(_) => continue,
                OpCode::Nop(val) => program.codes[i] = OpCode::Jmp(val),
                OpCode::Jmp(val) => program.codes[i] = OpCode::Nop(val),
            }
            match ExecutionContext::new().run(&program) {
                ExitReason::Loop => continue,
                ExitReason::Terminate => return program,
            }
        }
        panic!("no modification produces terminating program");
    }
}

impl OpCode {
    pub fn new(s: &str) -> OpCode {
        let fields = s.trim().split(' ').collect::<Vec<_>>();
        let val = fields[1]
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("can't parse: {}", fields[1]));
        match fields[0] {
            "nop" => OpCode::Nop(val),
            "acc" => OpCode::Acc(val),
            "jmp" => OpCode::Jmp(val),
            _ => panic!("unrecognized opcode: {}", fields[0]),
        }
    }
}

impl ExecutionContext {
    pub fn new() -> ExecutionContext {
        ExecutionContext {
            program_counter: 0,
            accumulator: 0,
            executed_instructions: HashSet::new(),
        }
    }

    pub fn run(&mut self, program: &Program) -> ExitReason {
        loop {
            self.executed_instructions.insert(self.program_counter);
            match program.codes[self.program_counter as usize] {
                OpCode::Nop(_) => self.program_counter += 1,
                OpCode::Acc(num) => {
                    self.accumulator += num;
                    self.program_counter += 1;
                }
                OpCode::Jmp(num) => {
                    self.program_counter += num;
                }
            }
            if self.executed_instructions.contains(&self.program_counter) {
                return ExitReason::Loop;
            }
            if self.program_counter < 0 {
                panic!("program counter goes negative!");
            }
            if self.program_counter >= program.codes.len() as i32 {
                return ExitReason::Terminate;
            }
        }
    }

    pub fn accumulator(&self) -> i32 {
        self.accumulator
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_run_to_first_loop() {
        let program = Program::new(TEST_INPUT);
        let mut ctx = ExecutionContext::new();
        ctx.run(&program);
        assert_eq!(5, ctx.accumulator());
    }

    #[test]
    fn test_modify_to_terminate() {
        let program = Program::new(TEST_INPUT);
        let program = program.modify_to_terminate();
        let mut ctx = ExecutionContext::new();
        ctx.run(&program);
        assert_eq!(8, ctx.accumulator());
    }
}
