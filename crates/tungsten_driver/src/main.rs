use std::{fs::File, path::Path};

use anyhow::{bail, Context, Result};
use args::{get_command, Command};
use memmap2::Mmap;
use tungsten_context::CompilerContext;
use tungsten_lexer::Lexer;

mod args;

fn read_file(file_path: &Path) -> Result<String> {
    let file = File::open(file_path).context("failed to open file")?;
    let src = unsafe { Mmap::map(&file).context("failed to read file into memory")? };
    let bytes = &src[..];

    Ok(String::from_utf8_lossy(bytes).to_string())
}

fn check_path_exists(path: &Path, context: &str) -> Result<()> {
    match path.try_exists() {
        Ok(true) => Ok(()),
        Ok(false) => bail!("{context} does not exist: {path:?}"),
        Err(e) => bail!("Failed to check existance of {context}: {e:?}"),
    }
}

fn create_context<'a>(
    file_path: &'a Path,
    source_code: &'a str,
    out_dir: &'a Path,
    opt_level: u8,
) -> CompilerContext<'a> {
    let mut ctx = CompilerContext::new(file_path, source_code, out_dir);
    ctx.set_opt_level(opt_level);

    ctx
}

fn main() -> Result<()> {
    let command = get_command();

    match command {
        Command::Build {
            file_name,
            opt_level,
            out_dir,
        } => {
            check_path_exists(&file_name, "Input file")?;
            check_path_exists(&out_dir, "Output directory")?;

            if !file_name.is_file() {
                bail!("Input path does not point to a file: {file_name:?}")
            }

            if !out_dir.is_dir() {
                bail!("Output path does not point to a directory: {out_dir:?}")
            }

            let source = read_file(&file_name).context("failed to read file")?;

            let mut ctx = create_context(&file_name, &source, &out_dir, opt_level);
            let mut lexer = Lexer::new(&mut ctx, &source);
            let tokens = lexer.tokenize();

            println!("{tokens:#?}");
        }
    };

    Ok(())
}
