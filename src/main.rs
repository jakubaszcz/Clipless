use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, hotkey::{HotKey, Modifiers, Code}};
use winit::event_loop::EventLoop;
use selection::get_text;
use rusqlite::{Connection, Result};


fn main() {
    let event_loop = EventLoop::new().unwrap();
    let manager = GlobalHotKeyManager::new().unwrap();

    let connection = init_database().unwrap();

    let custom_copy_hotkey = HotKey::new(
        Some(Modifiers::CONTROL | Modifiers::ALT),
        Code::KeyC,
    );

    manager.register(custom_copy_hotkey).unwrap();

    let receiver = GlobalHotKeyEvent::receiver();

    event_loop.run(move |_event, _| {
        if let Ok(_event) = receiver.try_recv() {
            let selected_text = get_text();

            insert_clip(&connection, &selected_text);
        }
    }).unwrap();
}

fn init_database() -> Result<Connection> {
    let conn = Connection::open("clipboard.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS clips (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

fn insert_clip(conn: &Connection, content: &str) {
    conn.execute("INSERT INTO clips (content) VALUES (?)", &[content]).unwrap();
}