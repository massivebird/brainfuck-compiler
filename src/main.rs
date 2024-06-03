use std::io::Write;

mod inst;

fn main() -> std::io::Result<()> {
    let matches = clap::command!()
        .arg(
            clap::Arg::new("src")
                .required(true)
                .value_hint(clap::ValueHint::FilePath)
                .value_name("FILE")
                .value_parser(clap::value_parser!(String))
                .help("Brainfuck source code file path"),
        )
        .get_matches();

    let mut buf: Vec<u8> = Vec::new();

    write_header_boilerplate(&mut buf).unwrap();

    let bf = std::fs::read_to_string(matches.get_one::<String>("src").unwrap()).unwrap();

    let instructions = self::inst::Inst::from_str(&bf);

    for inst in instructions {
        match inst.kind {
            inst::Kind::IncPtr => writeln!(&mut buf, "addb r12, 1")?,
            inst::Kind::DecPtr => writeln!(&mut buf, "subb r12, 1")?,
            inst::Kind::IncByte => writeln!(&mut buf, "addb [r12], 1")?,
            inst::Kind::DecByte => writeln!(&mut buf, "subb [r12], 1")?,
            inst::Kind::WriteByte => {
                writeln!(&mut buf, "mov rax, 0")?; // syscall code for "read" => 0
                writeln!(&mut buf, "mov rdi, rax")?; // code for stdin => 0
                writeln!(&mut buf, "mov rsi, r12")?; // buffer w addr to write to
                writeln!(&mut buf, "mov rdx, 1")?; // number of chars to write
                writeln!(&mut buf, "syscall")?;
            }
            inst::Kind::PrintByte => {
                writeln!(&mut buf, "mov rax, 1")?; // syscall code for "write" => 1
                writeln!(&mut buf, "mov rdi, rax")?; // code for stdout => 1
                writeln!(&mut buf, "mov rsi, r12")?; // buffer w addr of first char
                writeln!(&mut buf, "mov rdx, 1")?; // number of chars to print
                writeln!(&mut buf, "syscall")?;
            }
            inst::Kind::LoopStart { loop_end_idx } => todo!(),
            inst::Kind::LoopEnd { loop_start_idx } => todo!(),
        }
    }

    write_footer_boilerplate(&mut buf)?;

    std::fs::write("out", buf)?;

    Ok(())
}

fn write_header_boilerplate(buf: &mut Vec<u8>) -> std::io::Result<()> {
    writeln!(buf, ".intel_syntax noprefix")?;

    writeln!(buf, ".data")?; // delimits start of program data
    writeln!(buf, ".equ SYS_EXIT, 60")?;
    writeln!(buf, ".equ EXIT_CODE_OK, 0")?;

    writeln!(buf, ".bss")?; // like .data, but these items are specially initialized
    writeln!(buf, ".lcomm ARRAY, 30000")?; // makes ARRAY, pointing to zero-initialized array
                                           // of 30k bytes

    writeln!(buf, ".text")?; // delimits start of program instructions
    writeln!(buf, ".global _start")?; // makes _start label visible to linker

    writeln!(buf, "_start:")?;
    writeln!(buf, "mov r12, offset ARRAY")?; // load the address of ARRAY into register

    Ok(())
}

fn write_footer_boilerplate(buf: &mut Vec<u8>) -> std::io::Result<()> {
    writeln!(buf, "mov rax, SYS_EXIT")?;
    writeln!(buf, "mov rdi, EXIT_CODE_OK")?;
    writeln!(buf, "syscall")?;

    Ok(())
}
