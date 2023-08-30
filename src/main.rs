use iced::mouse::Cursor;
use iced::theme::Theme;
use iced::widget::canvas::{self, Frame, Geometry, Path, Program, Stroke};
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
                limbs: vec![Limb::new(
                    Some(Point::new(20.0, 30.0)),
                    Some(Point::new(100.00, 100.0)),
                    100.0,
                    100.0,
                    Some(Box::new(Limb::new(None, None, 100.0, 100.0, None))),
                )],
            },
        }
    }
    fn title(&self) -> String {
        String::from("Hello")
    }
    fn update(&mut self, message: Self::Message) {
        match message {
            _ => {}
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
    limbs: Vec<Limb>,
}
#[derive(Debug)]
struct Limb {
    start_point: Option<Point>,
    end_point: Option<Point>,
    length: f32,
    alpha: f32,
    next: Option<Box<Limb>>,
}
impl Limb {
    // recursively update children coordinates based on previous
    fn update_children(&mut self) {
        match &mut self.next {
            Some(next) => {
                if let Some(previous_point) = self.end_point {
                    next.end_point = Some(previous_point);
                }
                next.update_children()
            }
            None => {}
        }
    }
    fn new(
        start_point: Option<Point>,
        end_point: Option<Point>,
        length: f32,
        alpha: f32,
        next: Option<Box<Limb>>,
    ) -> Limb {
        let mut limb = Limb {
            start_point,
            end_point,
            length,
            alpha,
            next,
        };
        limb.update_children();
        limb
    }
    fn render(&self, frame: &mut Frame) {
        match self.start_point {
            Some(start_point) => match self.end_point {
                Some(end_point) => {
                    let line = Path::line(start_point, end_point);
                    let start = Path::circle(start_point, 10.0);
                    let end = Path::circle(end_point, 10.0);
                    frame.stroke(&line, Stroke::default().with_width(2.0));
                    frame.fill(&start, Color::BLACK);
                    frame.fill(&end, Color::BLACK);
                }
                None => {}
            },
            None => {}
        }
    }
}
impl Program<Message> for Circle {
    type State = ();
    fn update(
        &self,
        _state: &mut Self::State,
        _event: canvas::Event,
        _bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        // the function does nothing for now
        (canvas::event::Status::Ignored, None)
    }
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        // Defining a new frame
        let mut frame = Frame::new(renderer, bounds.size());
        // iterating through limbs and drawing them to the screen
        for l in self.limbs.iter() {
            l.render(&mut frame);
        }
        vec![frame.into_geometry()]
    }
}
