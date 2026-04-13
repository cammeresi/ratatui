/// A Ratatui example that shows how to create hyperlinks in the terminal using [OSC 8].
///
/// This example runs with the Ratatui library code in the branch that you are currently
/// reading. See the [`latest`] branch for the code which works with the most recent Ratatui
/// release.
///
/// [`latest`]: https://github.com/ratatui/ratatui/tree/latest
/// [OSC 8]: https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda
use color_eyre::Result;
use crossterm::event;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};

fn main() -> Result<()> {
    color_eyre::install()?;

    let line = Line::from(vec![
        Span::raw("Example "),
        Span::raw("hyperlink")
            .blue()
            .hyperlink(Some("https://example.com")),
    ]);

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| frame.render_widget(&line, frame.area()))?;
            if event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })
}
