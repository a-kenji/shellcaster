use std::collections::HashMap;
use pancurses::Input;

/// Enum delineating all actions that may be performed by the user, and
/// thus have keybindings associated with them.
#[derive(Debug)]
pub enum UserAction {
    Left,
    Right,
    Up,
    Down,

    AddFeed,
    Sync,
    SyncAll,

    Play,
    MarkPlayed,
    MarkAllPlayed,

    Download,
    DownloadAll,
    Delete,
    DeleteAll,
    Remove,
    RemoveAll,

    Search,
    Quit,
    Noop,
}

/// Wrapper around a hash map that keeps track of all keybindings. Multiple
/// keys may perform the same action, but each key may only perform one
/// action.
#[derive(Debug)]
pub struct Keybindings {
    map: HashMap<String, UserAction>,
}

impl Keybindings {
    pub fn new() -> Keybindings {
        return Keybindings {
            map: HashMap::new(),
        };
    }

    /// Takes an Input object from pancurses and returns the associated
    /// user action, if one exists.
    pub fn get_from_input(&self, input: &Input) -> Option<&UserAction> {
        match input_to_str(input) {
            Some(code) => {
                self.map.get(&code)
            },
            None => None,
        }
    }

    /// Takes a string representing a keycode and returns the associated
    /// user action, if one exists. 
    pub fn get_from_str(&self, code: &str) -> Option<&UserAction> {
        return self.map.get(&code.to_string());
    }

    /// Inserts a new keybinding into the hash map.
    pub fn insert(&mut self, code: &str, action: UserAction) -> bool {
        // ensures that earlier settings take priority over later settings
        if self.map.contains_key(&code.to_string()) {
            return false;
        } else {
            self.map.insert(code.to_string(), action);
            return true;
        }
    }
}

/// Helper function converting a pancurses Input object to a unique string
/// representing that input.
/// This function is a bit ridiculous, given that 95% of keyboards probably
/// don't even have half these special keys, but at any rate...they're
/// mapped, if anyone wants them.
pub fn input_to_str(input: &Input) -> Option<String> {
    let code = match input {
        Input::KeyCodeYes => "CodeYes",
        Input::KeyBreak => "Break",
        Input::KeyDown => "Down",
        Input::KeyUp => "Up",
        Input::KeyLeft => "Left",
        Input::KeyRight => "Right",
        Input::KeyHome => "Home",
        Input::KeyBackspace => "Backspace",
        Input::KeyF0 => "F0",
        Input::KeyF1 => "F1",
        Input::KeyF2 => "F2",
        Input::KeyF3 => "F3",
        Input::KeyF4 => "F4",
        Input::KeyF5 => "F5",
        Input::KeyF6 => "F6",
        Input::KeyF7 => "F7",
        Input::KeyF8 => "F8",
        Input::KeyF9 => "F9",
        Input::KeyF10 => "F10",
        Input::KeyF11 => "F11",  // F11 triggers KeyResize for me
        Input::KeyF12 => "F12",
        Input::KeyF13 => "F13",
        Input::KeyF14 => "F14",
        Input::KeyF15 => "F15",
        Input::KeyDL => "DL",
        Input::KeyIL => "IL",
        Input::KeyDC => "Del",
        Input::KeyIC => "Ins",
        Input::KeyEIC => "EIC",
        Input::KeyClear => "Clear",
        Input::KeyEOS => "EOS",
        Input::KeyEOL => "EOL",
        Input::KeySF => "S_Down",
        Input::KeySR => "S_Up",
        Input::KeyNPage => "PgDn",
        Input::KeyPPage => "PgUp",
        Input::KeySTab => "STab",  // this doesn't appear to be Shift+Tab
        Input::KeyCTab => "C_Tab",
        Input::KeyCATab => "CATab",
        Input::KeyEnter => "Enter",
        Input::KeySReset => "SReset",
        Input::KeyReset => "Reset",
        Input::KeyPrint => "Print",
        Input::KeyLL => "LL",
        Input::KeyAbort => "Abort",
        Input::KeySHelp => "SHelp",
        Input::KeyLHelp => "LHelp",
        Input::KeyBTab => "S_Tab",  // Shift+Tab
        Input::KeyBeg => "Beg",
        Input::KeyCancel => "Cancel",
        Input::KeyClose => "Close",
        Input::KeyCommand => "Command",
        Input::KeyCopy => "Copy",
        Input::KeyEnd => "End",
        Input::KeyExit => "Exit",
        Input::KeyFind => "Find",
        Input::KeyHelp => "Help",
        Input::KeyMark => "Mark",
        Input::KeyMessage => "Message",
        Input::KeyMove => "Move",
        Input::KeyNext => "Next",
        Input::KeyOpen => "Open",
        Input::KeyOptions => "Options",
        Input::KeyPrevious => "Previous",
        Input::KeyRedo => "Redo",
        Input::KeyReference => "Reference",
        Input::KeyRefresh => "Refresh",
        Input::KeyResume => "Resume",
        Input::KeyRestart => "Restart",
        Input::KeySave => "Save",
        Input::KeySBeg => "S_Beg",
        Input::KeySCancel => "S_Cancel",
        Input::KeySCommand => "S_Command",
        Input::KeySCopy => "S_Copy",
        Input::KeySCreate => "S_Create",
        Input::KeySDC => "S_Del",
        Input::KeySDL => "S_DL",
        Input::KeySelect => "Select",
        Input::KeySEnd => "S_End",
        Input::KeySEOL => "S_EOL",
        Input::KeySExit => "S_Exit",
        Input::KeySFind => "S_Find",
        Input::KeySHome => "S_Home",
        Input::KeySIC => "S_Ins",
        Input::KeySLeft => "S_Left",
        Input::KeySMessage => "S_Message",
        Input::KeySMove => "S_Move",
        Input::KeySNext => "S_PgDn",
        Input::KeySOptions => "S_Options",
        Input::KeySPrevious => "S_PgUp",
        Input::KeySPrint => "S_Print",
        Input::KeySRedo => "S_Redo",
        Input::KeySReplace => "S_Replace",
        Input::KeySRight => "S_Right",
        Input::KeySResume => "S_Resume",
        Input::KeySSave => "S_Save",
        Input::KeySSuspend => "S_Suspend",
        Input::KeySUndo => "S_Undo",
        Input::KeySuspend => "Suspend",
        Input::KeyUndo => "Undo",
        Input::KeyResize => "F11", // I'm marking this as F11 as well
        Input::KeyEvent => "Event",
        Input::KeyMouse => "Mouse",
        Input::KeyA1 => "A1",
        Input::KeyA3 => "A3",
        Input::KeyB2 => "B2",
        Input::KeyC1 => "C1",
        Input::KeyC3 => "C3",
        Input::Character(c) => {
            if c == &'\u{7f}' {
                "Backspace"
            } else if c == &'\u{1b}' {
                "Escape"
            } else if c == &'\n' {
                "Enter"
            } else if c == &'\t' {
                "Tab"
            } else {
                &c.to_string()[..]
            }
        },
        _ => "",
    };
    if code == "" {
        return None;
    } else {
        return Some(code.to_string());
    }
}