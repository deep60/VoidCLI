use std::{cell::Cell, char, collections::HashMap, fmt::format, usize};
use anyhow::Result;

use crate::parser::TerminalAction;

/// Default terminal colors (ANSI 16-color palette)
const DEFAULT_COLORS: [&str; 16] = [
    "#000000", // Black
    "#CC0000", // Red
    "#4E9A06", // Green
    "#C4A000", // Yellow
    "#3465A4", // Blue
    "#75507B", // Magenta
    "#06989A", // Cyan
    "#D3D7CF", // White
    "#555753", // Bright Black
    "#EF2929", // Bright Red
    "#8AE234", // Bright Green
    "#FCE94F", // Bright Yellow
    "#729FCF", // Bright Blue
    "#AD7FA8", // Bright Magenta
    "#34E2E2", // Bright Cyan
    "#EEEEEC", // Bright White
];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CellAttributes {
    /// Foreground color (ANSI color index or RGB)
    pub fg_color: Option<u32>,
    /// Background color (ANSI color index or RGB)
    pub bg_color: Option<u32>,
    /// Bold text
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub blink: bool,
    pub reverse: bool,
    pub hidden: bool,
    pub strikethrough: bool,
}

impl Default for CellAttributes {
    fn default() -> Self {
        Self {
            fg_color: Some(7),
            bg_color: Some(0),
            bold: false,
            italic: false,
            underline: false,
            blink: false,
            reverse: false,
            hidden: false,
            strikethrough: false,
        }
    }
}

/// Represents a cell in the terminal grid
#[derive(Debug, Clone)]
pub struct Cell {
    /// Character to display
    pub character: char,
    /// Cell attributes
    pub attributes: CellAttributes,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            character: ' ',
            attributes: CellAttributes::default(),
        }
    }
}

/// Represent the terminal grid/buffer
pub struct VirtualTerminal {
    /// The grid of cells
    grid: Vec<Vec<Cell>>,
    /// Terminal dimensions
    pub cols: usize,
    pub rows: usize,
    // Cursor position
    cursor_row: usize,
    cursor_col: usize,
    /// curernt attributes for new cells
    current_attributes: CellAttributes,
    /// saved cursor position
    saved_cursor_row: usize,
    saved_cursor_col: usize,
    // saved attributes
    saved_attributes: CellAttributes,
    // color palette
    color_palette: Vec<String>,
    // Terminal title
    pub title: String,
    // Scroll region (top, botto)
    scroll_region: (usize, usize),
    // Alternate screen buffer flag
    alt_buffer_active: bool,
    // main screen buffer (when alt is active)
    main_grid: Option<Vec<Vec<Cell>>>,
}

impl VirtualTerminal {
    /// Create a new virtual terminal with specified dimensions
    pub fn new(cols: usize, rows: usize) -> Self {
        let mut color_palette = Vec::with_capacity(256);

        // Add the default 16 colors
        for color in DEFAULT_COLORS.iter() {
            color_palette.push(color.to_string());
        }

        // Add the 216 color cube (6x6x6)
        for r in 0..6 {
            for g in 0..6  {
                for b in 0..6  {
                   let red = if r > 0 { r * 40 + 55 } else { 0 };
                   let green = if g > 0 { g * 40 + 55 } else { 0 };
                   let blue = if b > 0 { b * 40 + 55 } else { 0 };
                   let hex = format!("#{:02X}{:02X}{:02X}", red, green, blue);
                   color_palette.push(hex);
                }
            }
        }

        // Add the 24 grayscale colors
        for i in 0..24  {
            let value = 8 + i * 10;
            let hex = format!("#{:02X}{:02X}{:02X}", value, value, value);
            color_palette.push(hex);
        }

        // Create the grid with default cells
        let mut grid = Vec::with_capacity(rows);
        for _ in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for _ in 0..cols {
                row.push(Cell::default());
            }
            grid.push(row);
        }

        Self {
            grid,
            cols,
            rows,
            cursor_row: 0,
            cursor_col: 0,
            current_attributes: CellAttributes::default(),
            saved_cursor_row: 0,
            saved_cursor_col: 0,
            saved_attributes: CellAttributes::default(),
            color_palette,
            title: String::from("Terminal"),
            scroll_region: (0, rows - 1),
            alt_buffer_active: false,
            main_grid: None,
        }
    }

    /// Resize the terminal
    pub fn resize(&mut self, cols: usize, rows: usize) {
        // Create a new grid with new dimensions
        let mut new_grid = Vec::with_capacity(rows);
        for i in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for j in 0..cols {
                if i < self.rows && j < self.cols {
                    // Copy existing cells
                    row.push(self.grid[i][j].clone());
                } else {
                    // Fill with default cells
                    row.push(Cell::default());
                }
            }
            new_grid.push(row);
        }

        self.grid = new_grid;
        self.cols = cols;
        self.rows = rows;

        // Adjust cursor if it's ouside the new dimensions
        self.cursor_row = self.cursor_row.min(rows - 1);
        self.cursor_col = self.cursor_col.min(cols - 1);

        // Adjust scroll region
        self.scroll_region = (0, rows - 1);
    }

    /// Process a terminal action
    pub fn process_action(&mut self, action: &TerminalAction) -> Result<()> {
        match action {
            TerminalAction::Print(byte) => {
                let c = *byte as char;
                self.put_char(c);
            }
            TerminalAction::Bell => {

            }
            TerminalAction::Backspace => {
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                }
            }

            TerminalAction::Tab => {
                // Move to next tab stop (usually 8 spaces)
                self.cursor_col = (self.cursor_col + 8) / 8 * 8;
                if self.cursor_col >= self.cols {
                    self.cursor_col = self.cols - 1;
                }
            }

            TerminalAction::LineFeed => {
                self.cursor_row += 1;
                if self.cursor_row > self.scroll_region.1 {
                    self.scroll_up(1);
                    self.cursor_row = self.scroll_region.1;
                }
            }

            TerminalAction::CarriageReturn => {
                self.cursor_col = 0;
            }

            TerminalAction::CursorUp(n) => {
                let n = *n as usize;
                if self.cursor_row >= n {
                    self.cursor_row -= n;
                } else {
                    self.cursor_row = 0;
                }
            }

            TerminalAction::CursorDown(n) => {
                let n = *n as usize;
                self.cursor_row = (self.cursor_row + n).min(self.rows - 1);
            }

            TerminalAction::CursorForward(n) => {
                let n = *n as usize;
                self.cursor_col = (self.cursor_col + n).min(self.cols - 1);
            }

            TerminalAction::CursorBackward(n) => {
                let n = *n as usize;
                if self.cursor_col >= n {
                    self.cursor_col -= n;
                } else {
                    self.cursor_col = 0;
                }
            }

            TerminalAction::CursorNextLine(n) => {
                let n = *n as usize;
                self.cursor_row = (self.cursor_row + n).min(self.rows - 1);
                self.cursor_col = 0;
            }

            TerminalAction::CursorPreviousLine(n) => {
                let n = *n as usize;
                if self.cursor_row >= n {
                    self.cursor_row -= n;
                } else {
                    self.cursor_row = 0;
                }

                self.cursor_col = 0;
            }

            TerminalAction::CursorPosition(row, col) => {
                let row = *row as usize;
                let col = *col as usize;
                // Terminal coordinates are 1-based, convert to 0-based
                self.cursor_row = row.saturating_sub(1).min(self.rows - 1);
                self.cursor_col = col.saturating_sub(1).min(self.cols - 1);
            }

            TerminalAction::EraseInDisplay(n) => {
                match n {
                    0 => {
                        // Erase from cursor to end of screen
                        self.erase_region(
                            self.cursor_row,
                            self.cursor_col,
                            self.rows - 1,
                            self.cols - 1,
                        );
                    },
                    1 => {
                        // Erase from start of screen to cursor 
                        self.erase_region(0, 0, self.cursor_row, self.cursor_col);
                    },

                    2 | 3 => {
                        // Erase entire screen to cursor
                        self.erase_region(0, 0, self.rows - 1, self.cols - 1);
                    },
                    _ => {}
                }
            }

            TerminalAction::EraseInLine(n) => {
                match n {
                    0 => {
                        // Erase cursor to end of line
                        self.erase_region(
                            self.cursor_row,
                            self.cursor_col,
                            self.cursor_row,
                            self.cols - 1,
                        );
                    }

                    1 => {
                        self.erase_region(self.cursor_row, 0, self.cursor_row, self.cursor_col);
                    }

                    2 => {
                        self.erase_region(self.cursor_row, 0, self.cursor_row, self.cols - 1);
                    }

                    _ => {}
                }
            }

            TerminalAction::SetGraphicsRendition(params) => {
                self.process_sgr(params);
            }

            TerminalAction::Reset => {
                // Reset terminal state
                self.current_attributes = CellAttributes::default();
                self.cursor_row = 0;
                self.cursor_col = 0;
                self.scroll_region = (0, self.rows - 1);

                //Clear screen
                self.erase_region(0, 0, self.rows - 1, self.cols -1);
            }

            TerminalAction::ScrollUp(n) => {
                let n = *n as usize;
                self.scroll_up(n);
            }
            TerminalAction::SetWindowTitle(title) => {
                self.title = title.clone();
            }
            TerminalAction::SetColorPalette(index, color) => {
                let index = *index as usize;
                if index < self.color_palette.len() {
                    self.color_palette[index] = color.clone();
                }
            }
        }

        Ok(())
    }

    /// Process SGR(Select Graphic Rendition) parameters
    fn process_sgr(&mut self, params: &[u32]) {
        if params.is_empty() {
            // SGR 0 (reset/normal) is implied when no parameters are given
            self.current_attributes = CellAttributes::default();
            return;
        }

        let mut i = 0;
        while i < params.len() {
            0 >= {
                // Reset all attributes
                self.current_attributes = CellAttributes::default();
            }

            1 => {
                // Bold
                self.current_attributes.bold = true;
            }

            3 => {
                // italic
                self.current_attributes.italic = true;
            }

            4 => {
                // underline
                self.current_attributes.underline = true;
            }

            5 => {
                // blink
                self.current_attributes.blink = true;
            }

            7 => {
                // reverse
                self.current_attributes.reverse = true;
            }

            8 => {
                // hidden
                self.current_attributes.hidden = true;
            }

            9 => {
                // strikethrough
                self.current_attributes.strikethrough = true;
            }

            21 => {
                // Double underline(or no bold, depending on terminal)
                self.current_attributes.bold = false;
            }

            22 => {
                // no bold
                self.current_attributes.bold = false;
            }

            23 => {
                // no italic
                self.current_attributes.italic = false;
            }

            24 => {
                // no underline
                self.current_attributes.underline = false;
            }

            25 => {
                // no blink
                self.current_attributes.blink = false;
            }

            27 => {
                // no reverse
                self.current_attributes.reverse = false;
            }

            28 => {
                self.current_attributes.hidden = false;
            }

            29 => {
                self.current_attributes.strikethrough = false;
            }

            30..=37 => {
                // Foreground color(8 colors)
                self.current_attributes.fg_color = Some(params[i] - 30);
            }

            38 => {
                // Extended Foreground color
                if i + 1 < params.len() {
                    match params[i + 1] {
                        5 => {
                            // 8-bit color (256 colors)
                            if i + 2 < params.len() {
                                self.current_attributes.fg_color = Some(params[i + 2]);
                                i += 2;
                            }
                        }

                        2 => {
                            // 24-bit RGB colors
                            if i +  4 < params.len() {
                                // Convert RGB to a single integer
                                let r = params[i + 2];
                                let g = params[i + 3];
                                let b = params[i + 4];
                                let rgb = (r << 16) | (g << 8) | b;
                                self.current_attributes.fg_color = Some(rgb | 0x1000000);
                                i += 4;
                            }
                        }
                         _ => {}
                    }

                    i += 1;
                }
            }

            39 => {
                // Default Foreground colors
                self.current_attributes.fg_color = Some(7);
            }

            40..=47 => {
                // Background color (8 colors)
                self.current_attributes.bg_color = Some(params[i] - 40);
            }

            48 => {
                // Extended bg color
                if i + 1 < params.len() {
                    match params[i + 1] {
                        5 => {
                            // 8-bit color (256 color)
                            if i + 2 < params.len() {
                                self.current_attributes.bg_color = Some(params[i + 2]);
                                i += 2;
                            }
                        }

                        2 => {
                            if i +  4 < params.len() {
                                // Convert RGB to a single integer
                                let r = params[i + 2];
                                let g = params[i + 3];
                                let b = params[i + 4];
                                let rgb = (r << 16) | (g << 8) | b;
                                self.current_attributes.fg_color = Some(rgb | 0x1000000);
                                i += 4;
                            }
                        }

                        _ => {}
                    }

                    i += 1;
                }
            }

            49 => {
                // Default Background color
                self.current_attributes.bg_color = Some(0);
            }

            90..=97 => {
                // bright Background color
                self.current_attributes.fg_color = Some(params[i] - 90 + 8);
            }

            100..=107 => {
                self.current_attributes.bg_color = Some(params[i] - 100 + 8);
            }

            _ => {}
        }

        i += 1;
     }

    /// Put a character at the current cursor position and advance cursor
    fn put_char(&mut self, c: char) {
        if c == '\n' {
            // Line feed
            self.process_action(&TerminalAction::LineFeed).unwrap();
            return;
        } else if c == '\r' {
            self.process_action(&TerminalAction::CarriageReturn).unwrap();
            return;
        } else if c == '\t' {
            // Tab
            self.process_action(&TerminalAction::Tab).unwrap();
            return;
        } else if c == '\x08' {
            // Backspace
            self.process_action(&TerminalAction::Backspace).unwrap();
            return;
        } else if c == '\x07' {
            self.process_action(&TerminalAction::Bell).unwrap();
            return;
        }

        // Put character at current position
        if self.cursor_row < self.rows && self.cursor_col < self.cols {
            self.grid[self.cursor_row][self.cursor_col] = Cell {
                character: c,
                attributes: self.current_attributes.clone(),
            };
        }

        // Advance cursor
        self.cursor_col += 1;
        if self.cursor_col >= self.cols {
            self.cursor_col = 0;
            self.cursor_row += 1;
            if self.cursor_row > self.scroll_region.1 {
                self.scroll_up(1);
                self.cursor_row = self.scroll_region.1;
            }
        }
    }

    fn erase_region(&mut self, start_row: usize, start_col: usize, end_row: usize, end_col: usize) {
        let start_row = start_row.min(self.rows - 1);
        let start_col = start_col.min(self.cols - 1);
        let end_row = end_row.min(self.rows - 1);
        let end_col = end_col.min(self.cols - 1);

        for row in start_row..= end_row {
            let col_start = if row == start_row { start_col } else { 0 };
            let col_end = if row == end_row {
                end_col
            } else {
                self.cols - 1
            };

            for col in col_start..= col_end {
                self.grid[row][col] = Cell {
                    character: ' ',
                    attributes: self.current_attributes.clone(),
                };
            }
        }
    }

    /// Scroll the screen up by n lines
    fn scroll_up(&mut self, n: usize) {
        let (top, bottom) = self.scroll_region;
        let n = n.min(bottom - top + 1);

        if n == 0 {
            return;
        }

        // Move all lines up
        for row in top..(bottom + 1 - n)  {
            for col in 0..self.cols {
                self.grid[row][col] = self.grid[row + n][col].clone();
            }
        }

        // Clear the bottom n lines
        for row in (bottom + 1 - n)..= bottom {
            for col in 0..self.cols {
                self.grid[row][col] = Cell {
                    character: ' ',
                    attributes: self.current_attributes.clone(),
                };
            }
        }
    }

    /// Switch to alternate screen buffer
    pub fn use_alternate_buffer(&mut self, enable: bool) {
        if enable != self.alt_buffer_active {
            if enable {
                // Switch to alternate buffer
                let mut alt_grid = Vec::with_capacity(self.rows);
                for _ in 0..self.rows {
                    let mut row = Vec::with_capacity(self.cols);
                    for _ in 0..self.cols {
                        row.push(Cell::default());
                    }
                    alt_grid.push(row);
                }
                self.main_grid = Some(std::mem::replace(&mut self.grid, alt_grid));
            } else {
                // Switch back to main buffer
                if let Some(main_grid) = self.main_grid.take() {
                    self.grid = main_grid;
                }
            }
            self.alt_buffer_active = enable;
        }
    }

    /// Get the current cell at the specified position
    pub fn get_cell(&self, rows: usize, col: usize) -> Option<&Cell> {
        if row < self.rows && col < self.cols {
            Some(&self.grid[row][col])
        } else {
            None
        }
    }

    /// Get cursor position
    pub fn get_cursor_position(&self) -> (usize, usize) {
        (self.cursor_row, self.cursor_col)
    }

    /// Get a color from the palette
    pub fn get_color(&self, index: u32) -> String {
        // Check if this is an RGB color (flagged with 0x1000000)
        if index & 0x1000000 != 0 {
            let r = (index >> 16) & 0xFF;
            let g = (index >> 8) & 0xFF;
            let b = index & 0xFF;
            return format!("#{:02X}{:02X}{:02X}", r, g, b);
        }

        let index = index as usize;
        if index < self.color_palette.len() {
            self.color_palette[index].clone()
        } else {
            "#FFFFFF".to_string()
        }
    }
}


