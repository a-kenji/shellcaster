use super::panel::Panel;
use crate::keymap::{Keybindings, UserAction};
use super::colors::Colors;


#[derive(Debug)]
pub struct HelpWin{
    pub panel: (Panel, Panel),
}

impl HelpWin{
/// Creates a pancurses window with a help message for when users
/// start the program for the first time. Responsibility for managing
/// the window is given back to the main UI object.
pub fn make_help_win(colors: Colors, keymap: &Keybindings, n_row: i32, n_col: i32) -> HelpWin {
let quit_keys = keymap.keys_for_action(UserAction::Quit);


let quit_str = match quit_keys.len() {
    0 => "<missing>".to_string(),
    1 => format!("\"{}\"", &quit_keys[0]),
    2 => format!("\"{}\" or \"{}\"", quit_keys[0], quit_keys[1]),
    _ => {
        let mut s = "".to_string();
        for i in 0..quit_keys.len() {
            if i == quit_keys.len() - 1 {
                s = format!("{}, \"{}\"", s, quit_keys[i]);
            } else {
                s = format!("{}, or \"{}\"", s, quit_keys[i]);
            }
        }
        s
    }
};

// the warning on the unused mut is a function of Rust getting
// confused between panel.rs and mock_panel.rs
#[allow(unused_mut)]
let mut help_win_values = Panel::new(colors.clone(), "[Help]".into(), 0, n_row, n_col / 2, 0, 0);

let keys = keymap.get_keymap();


let mut row = 0;
row = help_win_values.write_wrap_line(row + 1, "Welcome to shellcaster!".to_string());


for (_, v) in keys.iter(){
    row = help_win_values.write_wrap_line(
        row + 1,
        format!("{:?}", &v)
    );
}

let _ = help_win_values.write_wrap_line(
    row + 1,
    "https://github.com/jeff-hughes/shellcaster".to_string(),
);

let mut help_win_keys = Panel::new(colors, "[ Keys ]".into(), 0, n_row, n_col, 0, n_col / 2 );
let mut row = 0;
row = help_win_keys.write_wrap_line(row + 1, "Welcome to shellcaster!".to_string());
for (k, _) in keys.iter(){
    row = help_win_keys.write_wrap_line(
        row + 1,
        format!("{:?}", &k)
    );
}

//help_win.border(

//)

return HelpWin{panel: (help_win_values, help_win_keys)};
}
}
