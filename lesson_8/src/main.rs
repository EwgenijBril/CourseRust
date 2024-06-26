#![warn(clippy::all, clippy::pedantic)]

use std::env::current_dir;
use std::fs::create_dir_all;
//use std::io::ErrorKind;
use std::path::PathBuf;
use std::io::Error as IOError;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let currennt_path = current_dir();

    // let mut target_path = match currennt_path {
    //     Ok(path) => path,
    //     Err(e) => panic!("Can't get current path: {e:?}"),
    // };

    let mut target_path = get_current_path()?;

    target_path.push("?");

    // create_dir_all(&target_path).expect("Can't create");

    // create_dir_all(&target_path).unwrap_or_else(|e| {
    //     panic!("Error: {e:?}");
    // });

    // create_dir_all(&target_path).unwrap();

    // match create_dir_all(&target_path) {
    //     Ok(()) => print!("Created {target_path:?}"),
    //     Err(e) => match e.kind() {
    //         ErroKind::InvalidFilename => {
    //             //
    //         },
    //         unknown_e => {
    //             panic!("Error: {unknown_e:?}");
    //         }
    //     },
    // };

    match create_dir_in(&target_path) {
        Ok(_) => print!("Created path!"),
        Err(e) => print!("fail: {e:?}"),
    };

    Ok(())
}

fn get_current_path() -> Result<PathBuf, IOError> {
    let path = current_dir()?;

    Ok(path)
}

// fn create_dir_in(target: &PathBuf) -> Result<String, IOError> {
//     match create_dir_all(target) {
//         Ok(_) => Ok(format!("{}", target.to_string_lossy())),
//         Err(e) => Err(e),
//     }

// }

fn create_dir_in(target: &PathBuf) -> Result<(), IOError> {
    create_dir_all(target)?;

    Ok(())
}
