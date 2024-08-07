pub mod counter;
pub mod game;

#[cfg(test)]
mod tests {
    use crate::game::Game;
    use std::path::PathBuf;
    use tenhou_parser::event_emitter::parse_file_iter;

    #[test]
    fn test() {
        let path = PathBuf::from(r#"D:\Projects\tenhou-log-manager\src-tauri\logs"#);
        let counters = Game::create_counters(vec!["Rikaka"]);
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
        let counter = counters.get("Rikaka").unwrap().borrow();
        println!(
            r#"
        总场数: {}
            一位率: {:.2}%
            二位率: {:.2}%
            三位率: {:.2}%
            四位率: {:.2}%
            被飞率: {:.2}%

            和了率: {:.2}%
            放铳率: {:.2}%
            自摸率: {:.2}%
            立直率: {:.2}%
            副露率: {:.2}%

            安定Rate: {:.2}
        "#,
            counter.matches,
            counter.rank1 as f32 / counter.matches as f32 * 100.0,
            counter.rank2 as f32 / counter.matches as f32 * 100.0,
            counter.rank3 as f32 / counter.matches as f32 * 100.0,
            counter.rank4 as f32 / counter.matches as f32 * 100.0,
            counter.tobi as f32 / counter.matches as f32 * 100.0,
            counter.wins as f32 / counter.rounds as f32 * 100.0,
            counter.loses as f32 / counter.rounds as f32 * 100.0,
            counter.win_tsumo as f32 / counter.wins as f32 * 100.0,
            counter.riichi as f32 / counter.rounds as f32 * 100.0,
            counter.total_furo as f32 / counter.rounds as f32 * 100.0,
            counter.tot_rate / counter.matches as f32 * 40.0
        );
    }
}
