use std::{io, env, fs};
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
            } else {
                f(file_u)?;
            }
        }
    }
    Ok(())
}

fn compile_scss(x: fs::DirEntry) -> io::Result<()> {
    if x.path().extension() == Some(&OsString::from("scss")) {
        let path = x.path().to_str().unwrap().to_owned();
        println!("{} {}", Colour::White.on(Colour::Green).paint("Compiling"), path);
        let result = grass::from_path(&path, &grass::Options::default()).unwrap();
        fs::write(path.replace(".scss", ".css"), result)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    visit_dirs(Path::new(&env::current_dir().unwrap()), compile_scss)?;
    Ok(())
}
