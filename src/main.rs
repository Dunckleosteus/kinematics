use iced::mouse::Cursor;
use iced::theme::Theme;
use iced::widget::canvas::path::lyon_path::Position;
use iced::widget::canvas::{self, Frame, Geometry, Path, Program};
use iced::widget::Canvas;
use iced::{Color, Element, Length, Point, Sandbox, Settings};

#[derive(Debug)]
pub enum Message {
    MouseMoved(Point),
    PlaceHolder,
    AddPoint(Point),
}

pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}
pub struct Hello {
    state: Circle, // <- Canvas as a field
}
impl Sandbox for Hello {
    type Message = Message;
    fn new() -> Hello {
        Hello {
            state: Circle {
                radius: 50.0,
                positions: vec![Point { x: 0.0, y: 0.0 }],
            },
        }
    }
    fn title(&self) -> String {
        String::from("Hello")
    }
    fn update(&mut self, message: Self::Message) {
        match message {
            Message::MouseMoved(x) => {}
            Message::AddPoint(x) => self.state.positions.push(x), // adding x to point list
            Message::PlaceHolder => {}
        }
    }
    fn view(&self) -> Element<'_, Self::Message> {
        Canvas::new(&self.state)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
// canvas
#[derive(Debug)]
struct Circle {
    radius: f32,
    positions: Vec<Point>,
}
impl Program<Message> for Circle {
    type State = ();
    fn update(
        &self,
        _state: &mut Self::State,
        event: canvas::Event,
        bounds: iced::Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        let cursor_position = if let Some(position) = cursor.position_in(bounds) {
            position
        } else {
            return (canvas::event::Status::Ignored, None);
        };

        match event {
            canvas::Event::Mouse(mouse) => match mouse {
                iced::mouse::Event::ButtonPressed(button) => match button {
                    iced::mouse::Button::Left => (
                        canvas::event::Status::Captured,
                        Some(Message::AddPoint(cursor_position)),
                    ),
                    iced::mouse::Button::Right => {
                        (canvas::event::Status::Captured, Some(Message::PlaceHolder))
                    }
                    _ => (canvas::event::Status::Ignored, None),
                },
                _ => (canvas::event::Status::Ignored, None),
            },
            _ => (canvas::event::Status::Ignored, None),
        }
    }
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        for i in self.positions.iter() {
            let circle = Path::circle(*i, self.radius);
            frame.fill(&circle, Color::BLACK);
        }
        vec![frame.into_geometry()]
    }
}
