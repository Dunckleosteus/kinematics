use iced::mouse::Cursor;
use iced::theme::Theme;
use iced::widget::canvas::{self, Frame, Geometry, Path, Program, Stroke};
use iced::widget::{column, Canvas};
use iced::{Color, Element, Length, Point, Sandbox, Settings};

#[derive(Debug, Clone)]
pub enum Message {
    MouseMoved(Point),
    PlaceHolder,
    AddPoint(Point),
    MoveLimb(Point),
    RotateLimb(f32),
    CountLimbs,
}

pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}
pub struct Hello {
    number_of_limbs: i32,
    state: Circle, // <- Canvas as a field
}
fn get_number_of_limbs(app: &mut Hello) {
    app.number_of_limbs = 0;
    app.number_of_limbs += 1;
    while app.state.limbs.next.is_some() {
        println!("helloooo");
        get_number_of_limbs(app)
    }
}
impl Sandbox for Hello {
    type Message = Message;
    fn new() -> Hello {
        Hello {
            number_of_limbs: 0,
            state: Circle {
                limbs: Limb::new(
                    Some(Point::new(400.0, 300.0)),
                    100.0,
                    0.0,
                    Some(Box::new(Limb::new(
                        None,
                        50.0,
                        90.0,
                        Some(Box::new(Limb::new(None, 50.0, 90.0, None))),
                    ))),
                ),
            },
        }
    }
    fn title(&self) -> String {
        String::from("Hello")
    }
    fn update(&mut self, message: Self::Message) {
        match message {
            Message::MoveLimb(x) => {
                self.state.limbs.start_point = Some(x);
                self.state.limbs.calculate_b();
                self.state.limbs.update_children();
            }
            Message::RotateLimb(x) => {
                self.state.limbs.rotate_all(x);
            }
            Message::CountLimbs => get_number_of_limbs(self),
            _ => {}
        }
    }
    fn view(&self) -> Element<'_, Self::Message> {
        column![
            iced::widget::row![
                // move limb right
                iced::widget::button("Right").on_press(
                    if let Some(point) = self.state.limbs.start_point {
                        Message::MoveLimb(Point::new(point.x + 5.0, point.y))
                    } else {
                        Message::PlaceHolder
                    }
                ),
                // move limb left
                iced::widget::button("Left").on_press(
                    if let Some(point) = self.state.limbs.start_point {
                        Message::MoveLimb(Point::new(point.x - 5.0, point.y))
                    } else {
                        Message::PlaceHolder
                    }
                ),
                // move limb down
                iced::widget::button("Down").on_press(
                    if let Some(point) = self.state.limbs.start_point {
                        Message::MoveLimb(Point::new(point.x, point.y + 5.0))
                    } else {
                        Message::PlaceHolder
                    }
                ),
                // moves limb upwards
                iced::widget::button("Up").on_press(
                    if let Some(point) = self.state.limbs.start_point {
                        Message::MoveLimb(Point::new(point.x, point.y - 5.0))
                    } else {
                        Message::PlaceHolder
                    }
                ),
                iced::widget::button("Rotate Clockwise")
                    .on_press(Message::RotateLimb(self.state.limbs.alpha + 5.0)),
                iced::widget::button("Rotate Anti Clockwise")
                    .on_press(Message::RotateLimb(self.state.limbs.alpha - 5.0)),
                iced::widget::button("Get number of limbs").on_press(Message::CountLimbs),
            ]
            .width(Length::Fill)
            .padding(5.0),
            Canvas::new(&self.state)
                .width(Length::Fill)
                .height(Length::Fill)
        ]
        .align_items(iced::Alignment::Center)
        .into()
    }
}
// canvas
#[derive(Debug)]
struct Circle {
    limbs: Limb,
}
#[derive(Debug)]
struct Limb {
    start_point: Option<Point>,
    end_point: Option<Point>,
    length: f32,
    alpha: f32,
    // recursive
    next: Option<Box<Limb>>,
}
impl Limb {
    // recursively update children coordinates based on previous
    fn calculate_b(&mut self) {
        match self.start_point {
            Some(point) => {
                let end_point = Point::new(
                    point.x + (self.alpha.cos() * self.length),
                    point.y + (self.alpha.sin() * self.length),
                );
                self.end_point = Some(end_point);
            }
            None => {}
        }
    }
    fn rotate_all(&mut self, angle: f32) {
        self.alpha += angle.to_radians();
        self.calculate_b();
        self.update_children();
        if let Some(next) = &mut self.next {
            next.rotate_all(angle);
            next.calculate_b();
            next.update_children();
        }
    }
    fn update_children(&mut self) {
        match &mut self.next {
            Some(next) => {
                if let Some(previous_point) = self.end_point {
                    let end_point_x: f32 = previous_point.x + (next.alpha.cos() * next.length);
                    let end_point_y: f32 = previous_point.y + (next.alpha.sin() * next.length);
                    next.start_point = Some(previous_point);
                    next.end_point = Some(Point::new(end_point_x, end_point_y));
                }
                next.update_children()
            }
            None => {}
        }
    }
    fn new(start_point: Option<Point>, length: f32, alpha: f32, next: Option<Box<Limb>>) -> Limb {
        let mut limb = Limb {
            start_point,
            end_point: None, // this will be calculated later with calculate_b()
            length,
            alpha: alpha.to_radians(),
            next,
        };
        limb.calculate_b();
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
        };
        match &self.next {
            Some(next) => next.render(frame),
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
        self.limbs.render(&mut frame);
        vec![frame.into_geometry()]
    }
}
