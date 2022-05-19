use std::{io, env};
use std::fs::{self};
use std::path::Path;
use std::ffi::OsString;
use ansi_term::Colour;

fn visit_dirs<F>(dir: &Path, f: F) -> io::Result<()>
where F: Fn(fs::DirEntry) -> io::Result<()> + Copy {
    if dir.is_dir() {
        let files = fs::read_dir(dir)?;
        for file in files {
            let file_u = file?;
            if file_u.file_type()?.is_dir() {
                visit_dirs(&file_u.path(), f)?;
            }
            f(file_u)?;
        }
    }
    Ok(())
}

fn on_scss(x: fs::DirEntry) -> io::Result<()> {
    if x.path().extension() == Some(&OsString::from("scss")) {
        println!("{} {}", Colour::White.on(Colour::Green).paint("Compiling"), x.path().to_str().unwrap());
        let result = grass::from_path(x.path().to_str().unwrap(), &grass::Options::default()).unwrap();
        fs::write(x.path().to_str().unwrap().replace(".scss", ".css"), result)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    visit_dirs(Path::new(&env::current_dir().unwrap()), on_scss)?;
    Ok(())
}
