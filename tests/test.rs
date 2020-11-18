use std::error::Error;

use frankolang::{
    instructions, instructions::FrankolangInstruction, FrankolangScript,
};

#[test]
fn frankolang_script_test() -> Result<(), Box<dyn Error>> {
    let mut frankolang_script = make_script();
    frankolang_script.execute()?;

    Ok(())
}

fn make_script() -> FrankolangScript {
    let sender: [u8; 32] = [
        112, 148, 71, 14, 227, 9, 139, 138, 69, 35, 179, 226, 121, 149, 143,
        72, 47, 206, 135, 183, 129, 228, 30, 43, 181, 163, 131, 41, 186, 139,
        127, 254,
    ];
    let receiver: [u8; 32] = [
        84, 166, 104, 24, 141, 227, 141, 23, 32, 17, 249, 123, 90, 188, 111,
        179, 90, 3, 89, 144, 194, 17, 72, 237, 213, 102, 109, 247, 123, 163,
        228, 56,
    ];

    let instructions: Vec<Box<dyn FrankolangInstruction>> = vec![
        Box::new(instructions::CoinbaseTransaction::new(sender)),
        Box::new(instructions::Payment::new(sender, receiver, 100)),
    ];

    FrankolangScript::from_instructions(instructions)
}
