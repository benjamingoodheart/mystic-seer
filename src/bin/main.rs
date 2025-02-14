use mystic_seer::mystic_seer;
fn main() {
    let ms = mystic_seer::MysticSeer::new();
    ms.welcome();
    let _ = ms.prompt();
}
