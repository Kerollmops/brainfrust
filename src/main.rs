use std::env;
use std::fs::File;
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::path::Path;

fn generate_code(input: BufReader<File>, mut output: BufWriter<File>) -> io::Result<()> {
    writeln!(&mut output, "use std::io::{{Read, Write}};")?;
    writeln!(&mut output, "const MEMORY_LEN: usize = 30_000;")?;
    writeln!(&mut output, "fn main() {{")?;
    writeln!(&mut output, "let mut memory = [0u8; MEMORY_LEN];")?;
    writeln!(&mut output, "let mut ptr = 0;")?;
    writeln!(&mut output, "let mut stdout = std::io::stdout();")?;
    writeln!(&mut output, "let mut stdin = std::io::stdin();")?;

    for c in input.bytes() {
        let c = c?;
        match c {
            b'>' => writeln!(&mut output, "ptr = (ptr + 1) % MEMORY_LEN;")?,
            b'<' => writeln!(&mut output, "ptr = ptr.wrapping_sub(1) % MEMORY_LEN;")?,
            b'+' => writeln!(&mut output, "memory[ptr] = memory[ptr].wrapping_add(1);")?,
            b'-' => writeln!(&mut output, "memory[ptr] = memory[ptr].wrapping_sub(1);")?,
            b'.' => writeln!(&mut output, "stdout.write_all(&[memory[ptr]]).unwrap();")?,
            b',' => writeln!(&mut output, "stdin.read(&mut memory[ptr..ptr + 1]).unwrap();")?,
            b'[' => writeln!(&mut output, "while memory[ptr] != 0 {{")?,
            b']' => writeln!(&mut output, "}}")?,
            _    => (),
        }
    }

    writeln!(&mut output, "}}")?;
    Ok(())
}

fn main() -> io::Result<()> {
    for filename in env::args().skip(1) {
        let path = Path::new(&filename);

        let input = File::open(path)?;
        let input = BufReader::new(input);

        let output = File::create(path.with_extension("rs"))?;
        let output = BufWriter::new(output);

        generate_code(input, output)?;
    }

    Ok(())
}
