use advent_2020::game_console;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_08/input.txt").unwrap();
    let program = game_console::Program::new(&input);
    let mut ctx: game_console::ExecutionContext = Default::default();
    ctx.run(&program);
    println!("Part 1: {}", ctx.accumulator());

    let program = program.modify_to_terminate();
    let mut ctx: game_console::ExecutionContext = Default::default();
    ctx.run(&program);
    println!("Part 2: {}", ctx.accumulator());
}
