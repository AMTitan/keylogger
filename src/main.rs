use inputbot::*;
fn main() {
    KeybdKey::bind_all(|event| {
        println!("{:?}", event);
    });

    inputbot::handle_input_events();
}
