use std::{
    env,
    fs::{copy, create_dir_all, metadata, read_dir, remove_file},
    io,
    path::Path,
};

fn main() {
    let mut args = env::args();
    let from = args.by_ref().skip(1).next().expect("from");
    let to = args.next().expect("to");
    update(from.as_ref(), to.as_ref()).expect("update");
}

fn update(from: &Path, to: &Path) -> io::Result<()> {
    fn update_dir(from: &Path, to: &Path) -> io::Result<()> {
        loop {
            match metadata(to) {
                Ok(meta) if !meta.is_dir() => {
                    remove_file(to)?;
                }
                Ok(_) => break,
                Err(_) => break create_dir_all(to)?,
            }
        }

        for entry in read_dir(from)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let path = entry.path();

            if file_type.is_dir() {
                let name = path.file_name().expect("is a dir");
                update_dir(&path, &to.join(name))?;
            } else if file_type.is_file() {
                let name = path.file_name().expect("is a file");
                let to = to.join(name);
                update_file(&path, &to)?;
            }
        }

        Ok(())
    }

    fn update_file(from: &Path, to: &Path) -> io::Result<()> {
        let from_last = metadata(from).map(|meta| meta.modified().expect("should support"))?;

        metadata(to)
            .map(|meta| meta.modified().expect("should support"))
            .ok()
            .map(|to_last| to_last < from_last)
            .unwrap_or(true)
            .then(|| copy(from, to))
            .transpose()?;

        Ok(())
    }

    if metadata(from)?.is_file() {
        update_file(from, to)
    } else {
        update_dir(from, to)
    }
}
