use tui::backend::Backend;
use tui::layout::Alignment;
use tui::widgets::Paragraph;
use tui::Frame;

///
/// Gets a renderer for logs.
///
pub fn get_logs_renderer<'a, B: Backend>(
    logs: &'a Vec<String>,
    current_scroll: &'a mut i32,
) -> impl FnOnce(&mut Frame<B>) + 'a {
    move |f| {
        let paragraph = Paragraph::new(
            logs.iter()
                .skip(*current_scroll as usize)
                .take(f.size().height as usize)
                .map(|f| f.clone())
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .alignment(Alignment::Left);

        f.render_widget(paragraph, f.size());
    }
}
