use crate::state::{AppState, VimMode};
use ratzilla::event::{KeyCode, KeyEvent};
use tui_textarea::Input;

pub fn handle_keys(state: &mut AppState, key_event: KeyEvent) {
    match state.vim_mode {
        VimMode::Normal => handle_normal_mode(state, key_event),
        VimMode::Insert => handle_insert_mode(state, key_event),
    }

    state.check_dirty();
}

fn handle_normal_mode(state: &mut AppState, key_event: KeyEvent) {
    match key_event.code {
        // Enter insert mode
        KeyCode::Char('i') => {
            state.vim_mode = VimMode::Insert;
        }
        KeyCode::Char('a') => {
            state.vim_mode = VimMode::Insert;
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Forward);
        }
        KeyCode::Char('A') => {
            state.vim_mode = VimMode::Insert;
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::End);
        }
        KeyCode::Char('I') => {
            state.vim_mode = VimMode::Insert;
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Head);
        }
        KeyCode::Char('o') => {
            state.vim_mode = VimMode::Insert;
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::End);
            state.editor.textarea.insert_newline();
        }
        KeyCode::Char('O') => {
            state.vim_mode = VimMode::Insert;
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Head);
            state.editor.textarea.insert_newline();
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Up);
        }
        // Navigation
        KeyCode::Char('h') | KeyCode::Left => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Back);
        }
        KeyCode::Char('j') | KeyCode::Down => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Down);
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Up);
        }
        KeyCode::Char('l') | KeyCode::Right => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Forward);
        }
        KeyCode::Char('0') => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Head);
        }
        KeyCode::Char('$') => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::End);
        }
        KeyCode::Char('g') => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Top);
        }
        KeyCode::Char('G') => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Bottom);
        }
        // Delete line
        KeyCode::Char('d') => {
            state.editor.textarea.delete_line_by_head();
        }
        KeyCode::Char('u') => {
            state.editor.textarea.undo();
        }
        KeyCode::Char('r') if key_event.ctrl => {
            state.editor.textarea.redo();
        }
        _ => {}
    }
}

fn handle_insert_mode(state: &mut AppState, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => {
            state.vim_mode = VimMode::Normal;
        }
        _ => {
            let input = convert_key_event_to_input(key_event);
            state.editor.textarea.input(input);
        }
    }
}

fn convert_key_event_to_input(key_event: KeyEvent) -> Input {
    Input {
        key: convert_keycode(key_event.code),
        ctrl: key_event.ctrl,
        alt: key_event.alt,
        shift: key_event.shift,
    }
}

fn convert_keycode(code: KeyCode) -> tui_textarea::Key {
    match code {
        KeyCode::Char(c) => tui_textarea::Key::Char(c),
        KeyCode::Backspace => tui_textarea::Key::Backspace,
        KeyCode::Enter => tui_textarea::Key::Enter,
        KeyCode::Left => tui_textarea::Key::Left,
        KeyCode::Right => tui_textarea::Key::Right,
        KeyCode::Up => tui_textarea::Key::Up,
        KeyCode::Down => tui_textarea::Key::Down,
        KeyCode::Tab => tui_textarea::Key::Tab,
        KeyCode::Delete => tui_textarea::Key::Delete,
        KeyCode::Home => tui_textarea::Key::Home,
        KeyCode::End => tui_textarea::Key::End,
        KeyCode::PageUp => tui_textarea::Key::PageUp,
        KeyCode::PageDown => tui_textarea::Key::PageDown,
        KeyCode::Esc => tui_textarea::Key::Esc,
        _ => tui_textarea::Key::Null,
    }
}
