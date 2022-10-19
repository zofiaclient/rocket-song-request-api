#[macro_use]
extern crate rocket;

use std::collections::LinkedList;

use rocket::{Build, Rocket};
use std::sync::{Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

static SONG_QUEUE: Mutex<LinkedList<String>> = Mutex::new(LinkedList::new());

fn acquire_queue<'a>() -> MutexGuard<'a, LinkedList<String>> {
    SONG_QUEUE
        .lock()
        .expect("Unable to acquire lock on song queue because the Mutex was poisoned")
}

fn remove_song_timer() {
    while !acquire_queue().is_empty() {
        thread::sleep(Duration::from_secs(60));
        acquire_queue().pop_front();
    }
}

#[post("/add/<song_name>")]
fn add_song(song_name: String) -> String {
    let mut lock = acquire_queue();

    if lock.is_empty() {
        thread::spawn(remove_song_timer);
    }
    lock.push_back(song_name);

    format!("Song added. This song is in position {}.", lock.len())
}

#[get("/view")]
fn view() -> String {
    format!("{:?}", acquire_queue())
}

#[launch]
fn rocket() -> Rocket<Build> {
    Rocket::build()
        // Set the `/` route path as the base for our routes.
        // When we create our routes, we'll include them in the arguments for the `routes!` macro.
        .mount("/", routes![add_song, view])
}
