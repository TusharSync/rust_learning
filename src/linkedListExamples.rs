// use std::collections::LinkedList;

// #[derive(Debug)]
// struct Song {
//     title: String,
//     artist: String,
// }

// #[derive(Debug)]
// struct Playlist {
//     songs: LinkedList<Song>,
//     current_index: usize,
// }

// impl Playlist {
//     fn new() -> Self {
//         Playlist {
//             songs: LinkedList::new(),
//             current_index: 0,
//         }
//     }

//     fn add_song(&mut self, title: String, artist: String) {
//         let song: Song = Song { title, artist };
//         self.songs.push_back(song);
//     }

//     fn next(&mut self) -> Option<&Song> {
//         if self.songs.is_empty() {
//             return None;
//         }

//         self.current_index = (self.current_index + 1) % self.songs.len();
//         self.songs.iter().nth(self.current_index)
//     }

//     fn previous(&mut self) -> Option<&Song> {
//         if self.songs.is_empty() {
//             return None;
//         }

//         if self.current_index == 0 {
//             self.current_index = self.songs.len() - 1;
//         } else {
//             self.current_index -= 1;
//         }
//         self.songs.iter().nth(self.current_index)
//     }

//     fn current_song(&self) -> Option<&Song> {
//         self.songs.iter().nth(self.current_index)
//     }
// }

// fn main() {
//     let mut playlist: Playlist = Playlist::new();

//     // Adding some songs to the playlist
//     playlist.add_song("Song 1".to_string(), "Artist A".to_string());
//     playlist.add_song("Song 2".to_string(), "Artist B".to_string());
//     playlist.add_song("Song 3".to_string(), "Artist C".to_string());

//     // Viewing the current song
//     if let Some(song) = playlist.current_song() {
//         println!("Currently Playing: {} by {}", song.title, song.artist);
//     } else {
//         println!("No songs in the playlist.");
//     }

//     // Moving to the next song
//     if let Some(song) = playlist.next() {
//         println!("Next Song: {} by {}", song.title, song.artist);
//     }

//     // Moving to the previous song
//     if let Some(song) = playlist.previous() {
//         println!("Previous Song: {} by {}", song.title, song.artist);
//     }

//     // Trying to go beyond the playlist to check looping
//     if let Some(song) = playlist.next() {
//         println!("Next Song: {} by {}", song.title, song.artist);
//     }

//     if let Some(song) = playlist.next() {
//         println!("Next Song: {} by {}", song.title, song.artist);
//     }

//     if let Some(song) = playlist.next() {
//         println!("Next Song: {} by {}", song.title, song.artist);
//     }

//     if let Some(song) = playlist.next() {
//         println!("Next Song: {} by {}", song.title, song.artist);
//     }

//     if let Some(song) = playlist.next() {
//         println!("Next Song: {} by {}", song.title, song.artist);
//     }

//     if let Some(song) = playlist.next() {
//         println!("Next Song: {} by {}", song.title, song.artist);
//     }
// }


// use std::collections::LinkedList;

// #[allow(dead_code)]
// fn main() {
//     #[derive(Debug)]
//     struct Task {
//         description: String,
//         priority: u32,
//     }

//     #[derive(Debug)]
//     struct TaskScheduler {
//         tasks: LinkedList<Task>,
//     }

//     impl TaskScheduler {
//         fn new() -> Self {
//             TaskScheduler {
//                 tasks: LinkedList::new(),
//             }
//         }

//         fn add_task(&mut self, description: String, priority: u32) {
//             let task: Task = Task {
//                 description,
//                 priority,
//             };
//             self.tasks.push_back(task);
//         }

//         fn execute_task(&mut self) -> Option<Task> {
//             self.tasks.pop_front()
//         }

//         fn peek_next_task(&self) -> Option<&Task> {
//             self.tasks.front()
//         }
//     }
//     let mut scheduler: TaskScheduler = TaskScheduler::new();

//     scheduler.add_task("Write report".to_string(), 1);
//     scheduler.add_task("Fix bugs".to_string(), 2);
//     scheduler.add_task("Attend meeting".to_string(), 3);

//     println!("Next Task: {:?}", scheduler.peek_next_task());

//     while let Some(task) = scheduler.execute_task() {
//         println!("Executing Task: {:?}", task);
//     }
// }


// use std::collections::LinkedList;

// #[derive(Debug)]
// struct BrowserHistory {
//     history: LinkedList<String>,
//     future: LinkedList<String>,
// }

// impl BrowserHistory {
//     fn new() -> Self {
//         BrowserHistory {
//             history: LinkedList::new(),
//             future: LinkedList::new(),
//         }
//     }

//     fn visit(&mut self, page: String) {
//         self.history.push_back(page);
//         self.future.clear(); // Clear the forward history on a new visit
//     }

//     fn back(&mut self) -> Option<String> {
//         if let Some(current) = self.history.pop_back() {
//             self.future.push_front(current.clone());
//             Some(current)
//         } else {
//             None
//         }
//     }

//     fn forward(&mut self) -> Option<String> {
//         if let Some(next) = self.future.pop_front() {
//             self.history.push_back(next.clone());
//             Some(next)
//         } else {
//             None
//         }
//     }

//     fn current(&self) -> Option<&String> {
//         self.history.back()
//     }
// }

// fn main() {
//     let mut browser: BrowserHistory = BrowserHistory::new();
//     browser.visit("home".to_string());
//     browser.visit("page1".to_string());
//     browser.visit("page2".to_string());

//     println!("Current Page: {:?}", browser.current());

//     browser.back();
//     println!("After Back, Current Page: {:?}", browser.current());

//     browser.forward();
//     println!("After Forward, Current Page: {:?}", browser.current());

//     browser.visit("page3".to_string());
//     println!("After Visiting a New Page, Current Page: {:?}", browser.current());
// }


