use std::io::stdout;

use crossterm::ExecutableCommand;

use crate::style::LoadingSpinnerStyle;

/// Draws a loading spinner that updates when you call tick.
///
/// # Example Usage
/// ```no_run
/// use progressed::LoadingSpinner;
/// use std::{thread, time::Duration};
///
/// let mut spinner = LoadingSpinner::default()
///     .set_title("Loading... ");
///
/// loop {
///     spinner.tick();
///     thread::sleep(Duration::from_millis(100));
/// }
///
/// ```
pub struct LoadingSpinner {
    style: LoadingSpinnerStyle,
    title: String,
    current_index: usize,
    start_pos: (u16, u16),
}

impl Default for LoadingSpinner {
    fn default() -> Self {
        let start_pos = crossterm::cursor::position().unwrap_or((0, 0));
        Self {
            style: LoadingSpinnerStyle::default(),
            title: String::new(),
            current_index: 0,
            start_pos,
        }
    }
}

impl LoadingSpinner {
    pub fn tick(&mut self) {
        let title = &self.title;
        let symbols = self.style.get_spinner_symbols();
        let symbol = match symbols.get(self.current_index) {
            Some(symbol) => symbol,
            None => {
                self.current_index = 0;
                if let Some(symbol) = symbols.get(0) {
                    symbol
                } else {
                    &' '
                }
            }
        };

        if stdout().execute(crossterm::cursor::Hide).is_ok() {}

        let (x, y) = self.start_pos;
        if let Ok(_) = stdout().execute(crossterm::cursor::MoveTo(x, y)) {}

        print!("{title}{symbol}");
        self.current_index += 1;
    }

    pub fn set_style(mut self, style: LoadingSpinnerStyle) -> Self {
        self.style = style;
        self
    }

    pub fn set_title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        self
    }
}
