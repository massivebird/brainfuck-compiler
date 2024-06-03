use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut buf: Vec<u8> = Vec::new();

    write_header_boilerplate(&mut buf).unwrap();

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
    writeln!(buf, "mov rax, SYS_EXIT")?;
    writeln!(buf, "mov rdi, EXIT_CODE_OK")?;
    writeln!(buf, "syscall")?;

    Ok(())
}
