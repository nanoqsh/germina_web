use std::{
    env,
    fs::{create_dir, metadata},
};

fn main() {
    let name = env::args().nth(1).expect("name");
    if let Err(_) = metadata(&name) {
        create_dir(&name).expect("create dir");
    }
}
