// Terminal parser implementation
// Handles parsing of terminal output data and escape sequences

use anyhow::Result;

/// Terminal parser that processes and interprets escape sequences
pub struct TerminalParser {
    // Parser state
    state: ParserState,
    // Current escape sequence being built
    escape_buffer: Vec<u8>,
    // Max size of escape buffer to prevent overflow
    max_escape_len: usize,
}

/// Enum representing different parser states
enum ParserState {
    /// Normal processing state
    Normal,
    /// Processing an escape sequence
    Escape,
    /// Processing an OSC (Operating System Command)
    Osc,
    /// Processing a CSI (Control Sequence Introducer)
    Csi,
}

impl TerminalParser {
    /// Create a new terminal parser
    pub fn new() -> Self {
        Self {
            state: ParserState::Normal,
            escape_buffer: Vec::with_capacity(128),
            max_escape_len: 1024,
        }
    }

    /// Parse terminal output data
    /// Returns processed data and actions to perform
    pub fn parse(&mut self, data: &[u8]) -> Result<Vec<TerminalAction>> {
        let mut actions = Vec::new();

        for &byte in data {
            match self.state {
                ParserState::Normal => {
                    match byte {
                        // ESC character
                        0x1b => {
                            self.escape_buffer.clear();
                            self.escape_buffer.push(byte);
                            self.state = ParserState::Escape;
                        }
                        // Handle other control characters
                        0x07 => actions.push(TerminalAction::Bell),
                        0x08 => actions.push(TerminalAction::Backspace),
                        0x09 => actions.push(TerminalAction::Tab),
                        0x0A => actions.push(TerminalAction::LineFeed),
                        0x0D => actions.push(TerminalAction::CarriageReturn),
                        // Normal printable character
                        _ => actions.push(TerminalAction::Print(byte)),
                    }
                }
                ParserState::Escape => {
                    self.escape_buffer.push(byte);
                    match byte {
                        // OSC - Operating System Command
                        b']' => {
                            self.state = ParserState::Osc;
                        }
                        // CSI - Control Sequence Introducer
                        b'[' => {
                            self.state = ParserState::Csi;
                        }
                        // Other escape sequences
                        _ => {
                            // Process simple escape sequence
                            if let Some(action) = self.process_simple_escape_sequence() {
                                actions.push(action);
                            }
                            self.state = ParserState::Normal;
                        }
                    }
                }
                ParserState::Csi => {
                    self.escape_buffer.push(byte);

                    // End of CSI sequence
                    if byte >= 0x40 && byte <= 0x7E {
                        if let Some(action) = self.process_csi_sequence() {
                            actions.push(action);
                        }
                        self.state = ParserState::Normal;
                    }

                    // Safety check for malformed sequences
                    if self.escape_buffer.len() > self.max_escape_len {
                        self.state = ParserState::Normal;
                    }
                }
                ParserState::Osc => {
                    self.escape_buffer.push(byte);

                    // End of OSC sequence (BEL or ST)
                    if byte == 0x07
                        || (byte == 0x5c
                            && self.escape_buffer.len() >= 2
                            && self.escape_buffer[self.escape_buffer.len() - 2] == 0x1b)
                    {
                        if let Some(action) = self.process_osc_sequence() {
                            actions.push(action);
                        }
                        self.state = ParserState::Normal;
                    }

                    // Safety check for malformed sequences
                    if self.escape_buffer.len() > self.max_escape_len {
                        self.state = ParserState::Normal;
                    }
                }
            }
        }

        Ok(actions)
    }

    fn process_simple_escape_sequence(&self) -> Option<TerminalAction> {
        if self.escape_buffer.len() < 2 {
            return None;
        }

        match self.escape_buffer[1] {
            b'A' => Some(TerminalAction::CursorUp(1)),
            b'B' => Some(TerminalAction::CursorDown(1)),
            b'C' => Some(TerminalAction::CursorForward(1)),
            b'D' => Some(TerminalAction::CursorBackward(1)),
            b'E' => Some(TerminalAction::CursorNextLine(1)),
            b'F' => Some(TerminalAction::CursorPreviousLine(1)),
            b'H' => Some(TerminalAction::CursorPosition(1, 1)),
            b'J' => Some(TerminalAction::EraseInDisplay(0)),
            b'K' => Some(TerminalAction::EraseInLine(0)),
            b'M' => Some(TerminalAction::ScrollUp(1)),
            b'c' => Some(TerminalAction::Reset),
            _ => None,
        }
    }

    fn process_csi_sequence(&self) -> Option<TerminalAction> {
        if self.escape_buffer.len() < 3 {
            return None;
        }

        let final_byte = *self.escape_buffer.last()?;
        let params_str =
            String::from_utf8_lossy(&self.escape_buffer[2..(self.escape_buffer.len() - 1)]);
        let params: Vec<u32> = params_str
            .split(';')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        match final_byte {
            b'm' => Some(TerminalAction::SetGraphicsRendition(params)),
            b'H' | b'f' => {
                let row = params.get(0).copied().unwrap_or(1);
                let col = params.get(1).copied().unwrap_or(1);
                Some(TerminalAction::CursorPosition(row, col))
            }
            b'J' => Some(TerminalAction::EraseInDisplay(
                params.get(0).copied().unwrap_or(0),
            )),
            b'K' => Some(TerminalAction::EraseInLine(
                params.get(0).copied().unwrap_or(0),
            )),
            b'A' => Some(TerminalAction::CursorUp(
                params.get(0).copied().unwrap_or(1),
            )),
            b'B' => Some(TerminalAction::CursorDown(
                params.get(0).copied().unwrap_or(1),
            )),
            b'C' => Some(TerminalAction::CursorForward(
                params.get(0).copied().unwrap_or(1),
            )),
            b'D' => Some(TerminalAction::CursorBackward(
                params.get(0).copied().unwrap_or(1),
            )),
            _ => None,
        }
    }

    fn process_osc_sequence(&self) -> Option<TerminalAction> {
        if self.escape_buffer.len() < 4 {
            return None;
        }

        let osc_data =
            String::from_utf8_lossy(&self.escape_buffer[2..(self.escape_buffer.len() - 1)]);

        if let Some(semicolon_pos) = osc_data.find(';') {
            let cmd = &osc_data[..semicolon_pos];
            let args = &osc_data[(semicolon_pos + 1)..];

            match cmd {
                "0" | "2" => Some(TerminalAction::SetWindowTitle(args.to_string())),
                "4" => {
                    // Color palette change
                    if let Some((color_index, color_value)) = args.split_once(';') {
                        if let (Ok(index), Some(color)) =
                            (color_index.parse::<u8>(), Some(color_value.to_string()))
                        {
                            return Some(TerminalAction::SetColorPalette(index, color));
                        }
                    }
                    None
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

/// Terminal actions that can be performed based on parsed terminal output
#[derive(Debug)]
pub enum TerminalAction {
    /// Print a character to the terminal
    Print(u8),
    /// Bell (alert) signal
    Bell,
    /// Backspace
    Backspace,
    /// Tab
    Tab,
    /// Line feed
    LineFeed,
    /// Carriage return
    CarriageReturn,
    /// Move cursor up by n rows
    CursorUp(u32),
    /// Move cursor down by n rows
    CursorDown(u32),
    /// Move cursor forward by n columns
    CursorForward(u32),
    /// Move cursor backward by n columns
    CursorBackward(u32),
    /// Move cursor to beginning of next line, n lines down
    CursorNextLine(u32),
    /// Move cursor to beginning of previous line, n lines up
    CursorPreviousLine(u32),
    /// Move cursor to position (row, column)
    CursorPosition(u32, u32),
    /// Erase in display (0=below, 1=above, 2=all, 3=saved lines)
    EraseInDisplay(u32),
    /// Erase in line (0=to right, 1=to left, 2=all)
    EraseInLine(u32),
    /// Set graphics rendition (colors, styles)
    SetGraphicsRendition(Vec<u32>),
    /// Reset terminal state
    Reset,
    /// Scroll up by n lines
    ScrollUp(u32),
    /// Set window title
    SetWindowTitle(String),
    /// Set color palette entry
    SetColorPalette(u8, String),
}

impl Default for TerminalParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_character_parsing() {
        let mut parser = TerminalParser::new();
        let actions = parser.parse(b"Hello").unwrap();

        assert_eq!(actions.len(), 5);
        if let TerminalAction::Print(b'H') = actions[0] {
            // Good
        } else {
            panic!("Expected Print('H') action");
        }
    }

    #[test]
    fn test_csi_sequence() {
        let mut parser = TerminalParser::new();
        // ESC[1;31m - Set text color to red
        let actions = parser.parse(b"\x1b[1;31m").unwrap();

        assert_eq!(actions.len(), 1);
        if let TerminalAction::SetGraphicsRendition(params) = &actions[0] {
            assert_eq!(params, &[1, 31]);
        } else {
            panic!("Expected SetGraphicsRendition action");
        }
    }
}
