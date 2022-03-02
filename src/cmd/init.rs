use std::io::Write;
use std::path::Path;
use std::fs::DirBuilder;
use log::trace;

use crate::model::config::Config;
use std::fs::File;
/// Init a Sloth repo.
/// 
/// Folder: before, after
/// sloth.json
pub fn init(pwd: &str, name: Option<&String>) {
    let name = match name {
        Some(s) => s,
        None => {
            println!("No name for the Sloth repository has been given.");
            std::process::exit(400)
        }
    };

    let rpath = &format!("/{pwd}/{name}");
    let path = Path::new(rpath);
    match DirBuilder::new().create(path) {
        Err(e) => { 
            trace!(" path: {path:?} | error : {e} ");
            println!("Sloth directory has already been initiated.")
        },
        Ok(_) => {
            let config = &format!("{rpath}/sloth.json");
            let mut config = match std::fs::File::create(config) {
                Ok(mut s) => {
                    match s.write_all(Config::init(name).as_bytes()) {
                        Ok(_) => trace!("Sloth.json has been created"),
                        Err(e) => trace!("Sloth.json creation has failed. Reason : {e} ")
                    };

                }
                Err(_) => todo!(),
            };
            // Create Config file
            // Create repo
        let bef = &format!("{rpath}/before");
        DirBuilder::new().create(bef);
        let af = &format!("{rpath}/after");
        DirBuilder::new().create(af);
        let test = &format!("{rpath}/test");
        DirBuilder::new().create(test);
        
        }
    }
    println!("Sloth repository named {name} has been initialized.")

}