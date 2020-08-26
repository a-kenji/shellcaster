use super::panel::Panel;
use crate::keymap::{Keybindings, UserAction};
use super::colors::Colors;

use std::collections::HashMap;


/// Simple wrapper struct
/// for the HelpWindow
#[derive(Debug)]
pub struct HelpWin{
    pub panel: (Panel, Panel),
}

impl HelpWin{
/// Creates a pancurses window with a help message,
/// that shows Keybindings and their usage.
/// Responsibility for managing
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
let mut help_win_values = Panel::new(colors.clone(), "[ Help ]".into(), 0, n_row, n_col / 2, 0, 0);

let mut row = 0;
row = help_win_values.write_wrap_line(row + 1, "Welcome to shellcaster!".to_string());

let keys = keymap.get_keymap();


// associate the actions with given keys
// because we want to group the keys
let mut action_keys = HashMap::new();
// key and value is here reversed to reason
// more easily in the if let
for (v, k) in &keys {
    if let Some(mut value) = action_keys.insert(k, vec![v]){
        // values must be mutable in order to append
        // because they will be consumed
        let mut value_vec = Vec::new();
        value_vec.append(&mut value);
        value_vec.append(&mut vec![v]);
        action_keys.insert(k, value_vec);
    };
}

// used for padding
let mut longest_value: usize = 0;
for val in action_keys.values(){
    let val_len = format!("{:?}",val);
    let val_len = val_len.len();
    if val_len > longest_value {
        longest_value = val_len;
    }
}

for (k, v) in &action_keys {
    let val_len = format!("{:?}",v);
    let val_len = val_len.len();
    row = help_win_values.write_wrap_line(
        row + 1,
        format!("{:?}:{:>width$}{:?}", &v, "".to_string(), &k, width = longest_value - val_len + 3));
}

let _ = help_win_values.write_wrap_line(
    row + 1,
    "https://github.com/jeff-hughes/shellcaster".to_string(),
);


// the warning on the unused mut is a function of Rust getting
// confused between panel.rs and mock_panel.rs
#[allow(unused_mut)]
let mut help_win_keys = Panel::new(colors.clone(), "[ Help ]".into(), 0, n_row, n_col, n_col / 2, 0);
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
