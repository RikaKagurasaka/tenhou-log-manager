use std::rc::Rc;
use maj_analyser::counter::Counter;
use maj_analyser::game::Game;
use tenhou_parser::event_emitter::parse_file_iter;

#[tauri::command]
pub fn parse_logs(id: String) -> Counter {
    let path = std::env::current_dir().unwrap().join("logs");
    let mut counters = Game::create_counters(vec![&id]);
    let mut game = Game::new(&counters);
    path.read_dir().unwrap().for_each(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();
        let _path_str = path.to_str().unwrap().to_string();
        if path.is_file() {
            parse_file_iter(path).for_each(|event| {
                game.on_event(event);
            });
        }
    });
    let rc = counters.remove(&id).unwrap();
    Rc::unwrap_or_clone(rc).into_inner()
}

#[tauri::command]
pub fn guess_user_id() -> Option<String> {
    tenhou_parser::event_emitter::guess_user_id(std::env::current_dir().unwrap().join("logs"))
}