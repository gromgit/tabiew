use ratatui::{prelude::*, widgets::*};

use crate::{
    app::App,
    theme::{Styler, Theme},
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(frame.size());
    
    // Set visible rows = table height - 1 (if header)
    app.visible_rows = layout[0].height - 1;

    
    frame.render_stateful_widget(&app.table, layout[0], &mut app.table_state);
    frame.render_widget(
        Line::default()
            .spans([
                Span::raw(format!(
                    "Row: {:<width$} ",
                    app.table_state.selected().map(|row| row + 1).unwrap_or(0),
                    width = app.rows.to_string().len()
                )),
                Span::raw(format!("Table Size: {} x {} ", app.rows, app.cols)),
            ])
            .alignment(Alignment::Right)
            .style(Theme::status_bar()),
        layout[1],
    );
}
