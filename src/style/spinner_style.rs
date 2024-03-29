use crate::defaults::DEFAULT_SPINNER_SURROUND;

/// Used to style a loading spinner.
///
/// # Example Usage
/// ```
/// use progressed::{LoadingSpinner, LoadingSpinnerStyle};
///
/// let style = LoadingSpinnerStyle::default()
///     .set_spinner_symbols(vec!['-', '+', '|']);
///
/// let spinner = LoadingSpinner::default().set_style(style);
/// ```
/// ## Styles
/// There a few defaults styles for easy to use good looking loading spinners.
///
/// ### Horizontal Blocks
/// Symbols: [▏ ▎ ▍ ▌ ▋ ▊ ▉ ▉ ▊ ▋ ▌ ▍ ▎]
///
/// ### Vertical Blocks
/// Symbols: [▁ ▂ ▃ ▄ ▅ ▆ ▇ █ ▇ ▆ ▅ ▄ ▃ ▁]
///
/// ### Rectangles
/// Symbols: [◰ ◳ ◲ ◱]
///
/// ### Triangles
/// Symbols: [◢ ◣ ◤ ◥]
///
/// ### Pipes
/// Symbols: [┤ ┘ ┴ └ ├ ┌ ┬ ┐]
///
/// ### Dots
/// Symbols: [⠁ ⠂ ⠄ ⡀ ⢀ ⠠ ⠐ ⠈]
///
/// ### Fancy
/// Symbols: [⣷ ⣯ ⣟ ⡿ ⢿ ⣻ ⣽ ⣾]
pub struct LoadingSpinnerStyle {
    spinner_symbols: Vec<char>,
}

impl Default for LoadingSpinnerStyle {
    fn default() -> Self {
        Self {
            spinner_symbols: DEFAULT_SPINNER_SURROUND.to_vec(),
        }
    }
}

impl LoadingSpinnerStyle {
    pub fn horizontal_blocks() -> Self {
        Self {
            spinner_symbols: vec![
                '▏', '▎', '▍', '▌', '▋', '▊', '▉', '▉', '▊', '▋', '▌', '▍', '▎',
            ],
        }
    }

    pub fn vertical_blocks() -> Self {
        Self {
            spinner_symbols: vec![
                '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█', '▇', '▆', '▅', '▄', '▃', '▁',
            ],
        }
    }

    pub fn rectangles() -> Self {
        Self {
            spinner_symbols: vec!['◰', '◳', '◲', '◱'],
        }
    }

    pub fn triangles() -> Self {
        Self {
            spinner_symbols: vec!['◢', '◣', '◤', '◥'],
        }
    }

    pub fn pipes() -> Self {
        Self {
            spinner_symbols: vec!['┤', '┘', '┴', '└', '├', '┌', '┬', '┐'],
        }
    }

    pub fn dots() -> Self {
        Self {
            spinner_symbols: vec!['⠁', '⠂', '⠄', '⡀', '⢀', '⠠', '⠐', '⠈'],
        }
    }

    pub fn fancy() -> Self {
        Self {
            spinner_symbols: vec!['⣷', '⣯', '⣟', '⡿', '⢿', '⣻', '⣽', '⣾'],
        }
    }

    pub fn get_spinner_symbols(&self) -> &Vec<char> {
        &self.spinner_symbols
    }

    pub fn set_spinner_symbols(mut self, symbols: Vec<char>) -> Self {
        self.spinner_symbols = symbols;
        self
    }
}
