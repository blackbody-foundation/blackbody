/*

    style.rs

    The MIT License (MIT)

    Copyright (c) 2017 Armin Ronacher <armin.ronacher@active-4.com>

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.

*/

use std::collections::BTreeSet;
use std::fmt;

/// Wraps an object for formatting for styling.
///
/// Example:
///
/// ```rust,no_run
/// # use console::style;
/// format!("Hello {}", style("World").cyan());
/// ```
///
/// This is a shortcut for making a new style and applying it
/// to a value:
///
/// ```rust,no_run
/// # use console::Style;
/// format!("Hello {}", Style::new().cyan().apply_to("World"));
/// ```
pub fn style<D>(val: D) -> StyledObject<D> {
    Style::new().apply_to(val)
}

/// A stored style that can be applied.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Style {
    fg: Option<Color>,
    bg: Option<Color>,
    fg_bright: bool,
    bg_bright: bool,
    attrs: BTreeSet<Attribute>,
}

impl Default for Style {
    fn default() -> Style {
        Style::new()
    }
}

impl Style {
    /// Returns an empty default style.
    pub fn new() -> Style {
        Style {
            fg: None,
            bg: None,
            fg_bright: false,
            bg_bright: false,
            attrs: BTreeSet::new(),
        }
    }
    /// Apply the style to something that can be displayed.
    pub fn apply_to<D>(&self, val: D) -> StyledObject<D> {
        StyledObject {
            style: self.clone(),
            val,
        }
    }

    /// Sets a foreground color.
    #[inline]
    pub fn fg(mut self, color: Color) -> Style {
        self.fg = Some(color);
        self
    }

    /// Sets a background color.
    #[inline]
    pub fn bg(mut self, color: Color) -> Style {
        self.bg = Some(color);
        self
    }

    /// Adds a attr.
    #[inline]
    pub fn attr(mut self, attr: Attribute) -> Style {
        self.attrs.insert(attr);
        self
    }

    #[inline]
    pub fn black(self) -> Style {
        self.fg(Color::Black)
    }
    #[inline]
    pub fn red(self) -> Style {
        self.fg(Color::Red)
    }
    #[inline]
    pub fn green(self) -> Style {
        self.fg(Color::Green)
    }
    #[inline]
    pub fn yellow(self) -> Style {
        self.fg(Color::Yellow)
    }
    #[inline]
    pub fn blue(self) -> Style {
        self.fg(Color::Blue)
    }
    #[inline]
    pub fn magenta(self) -> Style {
        self.fg(Color::Magenta)
    }
    #[inline]
    pub fn cyan(self) -> Style {
        self.fg(Color::Cyan)
    }
    #[inline]
    pub fn white(self) -> Style {
        self.fg(Color::White)
    }
    #[inline]
    pub fn grey(self) -> Style {
        self.fg(Color::Grey)
    }
    #[inline]
    pub fn color256(self, color: u8) -> Style {
        self.fg(Color::Color256(color))
    }

    #[inline]
    pub fn bright(mut self) -> Style {
        self.fg_bright = true;
        self
    }

    #[inline]
    pub fn on_black(self) -> Style {
        self.bg(Color::Black)
    }
    #[inline]
    pub fn on_grey(self) -> Style {
        self.bg(Color::Grey)
    }
    #[inline]
    pub fn on_red(self) -> Style {
        self.bg(Color::Red)
    }
    #[inline]
    pub fn on_green(self) -> Style {
        self.bg(Color::Green)
    }
    #[inline]
    pub fn on_yellow(self) -> Style {
        self.bg(Color::Yellow)
    }
    #[inline]
    pub fn on_blue(self) -> Style {
        self.bg(Color::Blue)
    }
    #[inline]
    pub fn on_magenta(self) -> Style {
        self.bg(Color::Magenta)
    }
    #[inline]
    pub fn on_cyan(self) -> Style {
        self.bg(Color::Cyan)
    }
    #[inline]
    pub fn on_white(self) -> Style {
        self.bg(Color::White)
    }
    #[inline]
    pub fn on_color256(self, color: u8) -> Style {
        self.bg(Color::Color256(color))
    }

    #[inline]
    pub fn on_bright(mut self) -> Style {
        self.bg_bright = true;
        self
    }

    #[inline]
    pub fn bold(self) -> Style {
        self.attr(Attribute::Bold)
    }
    #[inline]
    pub fn dim(self) -> Style {
        self.attr(Attribute::Dim)
    }
    #[inline]
    pub fn italic(self) -> Style {
        self.attr(Attribute::Italic)
    }
    #[inline]
    pub fn underlined(self) -> Style {
        self.attr(Attribute::Underlined)
    }
    #[inline]
    pub fn blink(self) -> Style {
        self.attr(Attribute::Blink)
    }
    #[inline]
    pub fn reverse(self) -> Style {
        self.attr(Attribute::Reverse)
    }
    #[inline]
    pub fn hidden(self) -> Style {
        self.attr(Attribute::Hidden)
    }
}

/// A terminal color.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    Grey,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Color256(u8),
}

impl Color {
    #[inline]
    fn ansi_num(self) -> usize {
        match self {
            Color::Black => 0,
            Color::Grey => 8,
            Color::Red => 1,
            Color::Green => 2,
            Color::Yellow => 3,
            Color::Blue => 4,
            Color::Magenta => 5,
            Color::Cyan => 6,
            Color::White => 7,
            Color::Color256(x) => x as usize,
        }
    }

    #[inline]
    fn is_color256(self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self {
            Color::Color256(_) => true,
            _ => false,
        }
    }
}

/// A terminal style attribute.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Attribute {
    Bold,
    Dim,
    Italic,
    Underlined,
    Blink,
    Reverse,
    Hidden,
}

impl Attribute {
    #[inline]
    fn ansi_num(self) -> usize {
        match self {
            Attribute::Bold => 1,
            Attribute::Dim => 2,
            Attribute::Italic => 3,
            Attribute::Underlined => 4,
            Attribute::Blink => 5,
            Attribute::Reverse => 7,
            Attribute::Hidden => 8,
        }
    }
}

/// A formatting wrapper that can be styled for a terminal.
#[derive(Clone)]
pub struct StyledObject<D> {
    style: Style,
    val: D,
}

impl<D> StyledObject<D> {
    /// Sets a foreground color.
    #[inline]
    pub fn fg(mut self, color: Color) -> StyledObject<D> {
        self.style = self.style.fg(color);
        self
    }

    /// Sets a background color.
    #[inline]
    pub fn bg(mut self, color: Color) -> StyledObject<D> {
        self.style = self.style.bg(color);
        self
    }

    /// Adds a attr.
    #[inline]
    pub fn attr(mut self, attr: Attribute) -> StyledObject<D> {
        self.style = self.style.attr(attr);
        self
    }

    #[inline]
    pub fn black(self) -> StyledObject<D> {
        self.fg(Color::Black)
    }
    #[inline]
    pub fn red(self) -> StyledObject<D> {
        self.fg(Color::Red)
    }
    #[inline]
    pub fn green(self) -> StyledObject<D> {
        self.fg(Color::Green)
    }
    #[inline]
    pub fn yellow(self) -> StyledObject<D> {
        self.fg(Color::Yellow)
    }
    #[inline]
    pub fn blue(self) -> StyledObject<D> {
        self.fg(Color::Blue)
    }
    #[inline]
    pub fn magenta(self) -> StyledObject<D> {
        self.fg(Color::Magenta)
    }
    #[inline]
    pub fn cyan(self) -> StyledObject<D> {
        self.fg(Color::Cyan)
    }
    #[inline]
    pub fn white(self) -> StyledObject<D> {
        self.fg(Color::White)
    }
    #[inline]
    pub fn grey(self) -> StyledObject<D> {
        self.fg(Color::Grey)
    }
    #[inline]
    pub fn color256(self, color: u8) -> StyledObject<D> {
        self.fg(Color::Color256(color))
    }

    #[inline]
    pub fn bright(mut self) -> StyledObject<D> {
        self.style = self.style.bright();
        self
    }

    #[inline]
    pub fn on_black(self) -> StyledObject<D> {
        self.bg(Color::Black)
    }
    #[inline]
    pub fn on_grey(self) -> StyledObject<D> {
        self.bg(Color::Grey)
    }
    #[inline]
    pub fn on_red(self) -> StyledObject<D> {
        self.bg(Color::Red)
    }
    #[inline]
    pub fn on_green(self) -> StyledObject<D> {
        self.bg(Color::Green)
    }
    #[inline]
    pub fn on_yellow(self) -> StyledObject<D> {
        self.bg(Color::Yellow)
    }
    #[inline]
    pub fn on_blue(self) -> StyledObject<D> {
        self.bg(Color::Blue)
    }
    #[inline]
    pub fn on_magenta(self) -> StyledObject<D> {
        self.bg(Color::Magenta)
    }
    #[inline]
    pub fn on_cyan(self) -> StyledObject<D> {
        self.bg(Color::Cyan)
    }
    #[inline]
    pub fn on_white(self) -> StyledObject<D> {
        self.bg(Color::White)
    }
    #[inline]
    pub fn on_color256(self, color: u8) -> StyledObject<D> {
        self.bg(Color::Color256(color))
    }

    #[inline]
    pub fn on_bright(mut self) -> StyledObject<D> {
        self.style = self.style.on_bright();
        self
    }

    #[inline]
    pub fn bold(self) -> StyledObject<D> {
        self.attr(Attribute::Bold)
    }
    #[inline]
    pub fn dim(self) -> StyledObject<D> {
        self.attr(Attribute::Dim)
    }
    #[inline]
    pub fn italic(self) -> StyledObject<D> {
        self.attr(Attribute::Italic)
    }
    #[inline]
    pub fn underlined(self) -> StyledObject<D> {
        self.attr(Attribute::Underlined)
    }
    #[inline]
    pub fn blink(self) -> StyledObject<D> {
        self.attr(Attribute::Blink)
    }
    #[inline]
    pub fn reverse(self) -> StyledObject<D> {
        self.attr(Attribute::Reverse)
    }
    #[inline]
    pub fn hidden(self) -> StyledObject<D> {
        self.attr(Attribute::Hidden)
    }
}

macro_rules! impl_fmt {
    ($name:ident) => {
        impl<D: fmt::$name> fmt::$name for StyledObject<D> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut reset = false;

                if let Some(fg) = self.style.fg {
                    if fg.is_color256() {
                        write!(f, "\x1b[38;5;{}m", fg.ansi_num())?;
                    } else if self.style.fg_bright {
                        write!(f, "\x1b[38;5;{}m", fg.ansi_num() + 8)?;
                    } else {
                        write!(f, "\x1b[{}m", fg.ansi_num() + 30)?;
                    }
                    reset = true;
                }
                if let Some(bg) = self.style.bg {
                    if bg.is_color256() {
                        write!(f, "\x1b[48;5;{}m", bg.ansi_num())?;
                    } else if self.style.bg_bright {
                        write!(f, "\x1b[48;5;{}m", bg.ansi_num() + 8)?;
                    } else {
                        write!(f, "\x1b[{}m", bg.ansi_num() + 40)?;
                    }
                    reset = true;
                }
                for attr in &self.style.attrs {
                    write!(f, "\x1b[{}m", attr.ansi_num())?;
                    reset = true;
                }

                fmt::$name::fmt(&self.val, f)?;
                if reset {
                    write!(f, "\x1b[0m")?;
                }
                Ok(())
            }
        }
    };
}

impl_fmt!(Binary);
impl_fmt!(Debug);
impl_fmt!(Display);
impl_fmt!(LowerExp);
impl_fmt!(LowerHex);
impl_fmt!(Octal);
impl_fmt!(Pointer);
impl_fmt!(UpperExp);
impl_fmt!(UpperHex);
