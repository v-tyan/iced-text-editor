use iced::widget::{column, container, horizontal_space, row, text, text_editor, Row, Text};
use iced::{Element, Length, Theme};

pub fn main() -> iced::Result {
    iced::application("Editor", update, view)
        .theme(|_| Theme::Dark)
        .centered()
        .run()
}

struct State {
    content: text_editor::Content,
}

impl Default for State {
    fn default() -> Self {
        Self {
            content: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

fn view(state: &State) -> Element<'_, Message> {
    let input = text_editor(&state.content)
        .placeholder("Type something here...")
        .on_action(Message::Edit)
        .height(Length::Fill);

    let position: Text = {
        let (row, column) = state.content.cursor_position();

        text(format!("{}:{}", row + 1, column + 1))
    };

    let status_bar: Row<Message> = row![horizontal_space(), position];

    return container(column![input, status_bar]).padding(10).into();
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Edit(action) => {
            state.content.perform(action);
        }
    }
}
