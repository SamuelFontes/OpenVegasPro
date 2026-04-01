use std::collections::HashMap;
use raylib::prelude::KeyboardKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    NewProject,
    OpenProject,
    SaveProject,
    PlayPause,
    Stop,
    GoToStart,
    SplitClip,
    Undo,
    Redo,
    UntieClips,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyCombo {
    pub key: KeyboardKey,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
}

pub struct KeyManager {
    pub bindings: HashMap<KeyCombo, Action>,
}

impl KeyManager {
    pub fn new() -> Self {
        let mut km = Self { bindings: HashMap::new() };
        km.load_vegas_defaults();
        km
    }

    pub fn load_vegas_defaults(&mut self) {
        use KeyboardKey::*;
        use Action::*;

        // ── Playback Controls (Vegas style) ──────────────────────────────────
        // Space: Play/Pause (Toggle)
        self.bind(KEY_SPACE, false, false, false, PlayPause);
        // Enter: Stop (Returns cursor to where it was before playback start)
        self.bind(KEY_ENTER, false, false, false, Stop);
        
        // ── Editing ──────────────────────────────────────────────────────────
        // S: Split clip at cursor
        self.bind(KEY_S, false, false, false, SplitClip);
        // U: Untie clips
        self.bind(KEY_U, false, false, false, UntieClips);
        
        // ── File / Project ───────────────────────────────────────────────────
        self.bind(KEY_S, true, false, false, SaveProject);
        self.bind(KEY_O, true, false, false, OpenProject);
        self.bind(KEY_N, true, false, false, NewProject);
        
        // ── History ──────────────────────────────────────────────────────────
        self.bind(KEY_Z, true, false, false, Undo);
        self.bind(KEY_Z, true, true, false, Redo);
        self.bind(KEY_Y, true, false, false, Redo);
    }

    pub fn bind(&mut self, key: KeyboardKey, ctrl: bool, shift: bool, alt: bool, action: Action) {
        self.bindings.insert(KeyCombo { key, ctrl, shift, alt }, action);
    }

    /// Returns a list of actions triggered this frame based on key presses.
    pub fn check_actions(&self, rl: &raylib::RaylibHandle) -> Vec<Action> {
        let mut actions = Vec::new();
        let ctrl = rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL);
        let shift = rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT);
        let alt = rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_ALT);

        for (combo, action) in &self.bindings {
            if combo.ctrl == ctrl && combo.shift == shift && combo.alt == alt {
                if rl.is_key_pressed(combo.key) {
                    actions.push(*action);
                }
            }
        }
        actions
    }
}
