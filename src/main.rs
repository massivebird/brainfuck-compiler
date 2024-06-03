use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut buf: Vec<u8> = Vec::new();

    writeln!(&mut buf, ".intel_syntax noprefix")?;

    writeln!(&mut buf, ".data")?; // delimits start of program data
    writeln!(&mut buf, ".equ SYS_EXIT, 60")?;
    writeln!(&mut buf, ".equ EXIT_CODE_OK, 0")?;

    writeln!(&mut buf, ".bss")?; // like .data, but these items are specially initialized
    writeln!(&mut buf, ".lcomm ARRAY, 30000")?; // makes ARRAY, pointing to zero-initialized array
                                                // of 30k bytes

    writeln!(&mut buf, ".text")?; // delimits start of program instructions
    writeln!(&mut buf, ".global _start")?; // makes _start label visible to linker

    writeln!(&mut buf, "_start:")?;
    writeln!(&mut buf, "mov rax, SYS_EXIT")?;
    writeln!(&mut buf, "mov rdi, EXIT_CODE_OK")?;
    writeln!(&mut buf, "syscall")?;

    std::fs::write("out", buf)?;

    Ok(())
}
