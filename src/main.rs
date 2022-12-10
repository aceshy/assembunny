mod program;

use program::Program;

fn main() -> Result<(), std::io::Error> {
    const FILE_PATH: &str = "./inputs/input.txt";
    let input = std::fs::read_to_string(FILE_PATH)?;

    Program::from_str(&input).run();

    Ok(())
}
