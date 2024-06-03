use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut buf: Vec<u8> = Vec::new();

    writeln!(&mut buf, ".intel_syntax noprefix")?;
    writeln!(&mut buf, ".global _start")?;
    writeln!(&mut buf, "_start:")?;

    std::fs::write("out", buf)?;

    Ok(())
}
