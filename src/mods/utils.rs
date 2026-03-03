use crossterm::event::KeyCode;
use ratatui::layout::Rect;
use ratatui::widgets::ListState;

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let padding_x = r
        .width
        .saturating_sub(r.width.saturating_mul(percent_x) / 100)
        / 2;
    let padding_y = r
        .height
        .saturating_sub(r.height.saturating_mul(percent_y) / 100)
        / 2;
    Rect::new(
        r.x + padding_x,
        r.y + padding_y,
        r.width.saturating_sub(padding_x * 2),
        r.height.saturating_sub(padding_y * 2),
    )
}

#[cfg(feature = "wayland_support")]
pub fn set_clipboard_content(content: &str) -> Result<(), String> {
    use wl_clipboard_rs::copy::{MimeType, Options, Source};
    let opts = Options::new();
    opts.copy(
        Source::Bytes(content.to_string().into_bytes().into()),
        MimeType::Autodetect,
    )
    .map_err(|_| "Failed to copy content to clipboard".to_string())?;
    Ok(())
}

#[cfg(not(feature = "wayland_support"))]
pub fn set_clipboard_content(content: &str) -> Result<(), String> {
    let clipboard =
        x11_clipboard::Clipboard::new().map_err(|_| "Failed to access clipboard".to_string())?;
    clipboard
        .store(
            clipboard.setter.atoms.clipboard,
            clipboard.setter.atoms.utf8_string,
            content,
        )
        .map_err(|_| "Failed to copy content to clipboard".to_string())?;
    Ok(())
}

pub fn clear_clipboard() {
    set_clipboard_content("").ok();
}

pub fn navigate_list(list_state: &mut ListState, total_options: usize, key_code: KeyCode) {
    if let Some(selected) = list_state.selected() {
        match key_code {
            KeyCode::Up => {
                if selected > 0 {
                    list_state.select(Some(selected - 1));
                }
            }
            KeyCode::Down => {
                if selected < total_options - 1 {
                    list_state.select(Some(selected + 1));
                }
            }
            _ => {}
        }
    }
}
