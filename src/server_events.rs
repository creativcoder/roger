use inotify::INotify;
use inotify::ffi::*;
use std::path::Path;

pub fn filesys_events() {
	let mut ino = INotify::init().unwrap();
    ino.add_watch(Path::new("/home/someuser"), IN_MODIFY | IN_CREATE | IN_DELETE).unwrap();
    loop {
        let events = ino.wait_for_events().unwrap();
        for event in events.iter() {
            if event.is_create() {
                if event.is_dir() {
                    println!("The directory \"{}\" was created.", event.name);       
                } else {
                    println!("The file \"{}\" was created.", event.name);
                }
            } else if event.is_delete() {
                if event.is_dir() {
                    println!("The directory \"{}\" was deleted.", event.name);       
                } else {
                    println!("The file \"{}\" was deleted.", event.name);
                }
            } else if event.is_modify() {
                if event.is_dir() {
                    println!("The directory \"{}\" was modified.", event.name);
                } else {
                    println!("The file \"{}\" was modified.", event.name);
                }
            }
        }
    }
}