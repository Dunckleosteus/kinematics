use iced::mouse::Cursor;
use iced::theme::Theme;
use iced::widget::canvas::{self, stroke, Frame, Geometry, Path, Program, Stroke};
use iced::widget::{column, Canvas};
use iced::{Color, Element, Length, Point, Sandbox, Settings};

#[derive(Debug, Clone)]
pub enum Message {
    MouseMoved(Point),
    PlaceHolder,
    Move(Direction),
    AddPoint(Point),
    RotateLimb(f32),
    CountLimbs,
}
#[derive(Debug, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
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
                limbs: Limb::new(vec![
                    Segment::new(Some(Point::new(100.0, 100.0)), 100.0, 30.0),
                    Segment::new(None, 100.0, 0.0),
                    Segment::new(None, 100.0, 30.0),
                ]),
            },
        }
    }
    fn title(&self) -> String {
        String::from("Hello")
    }
    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Move(x) => self.state.limbs.move_origin(x),
            Message::RotateLimb(x) => {
                //self.state.limbs.rotate_all(x);
            }
            Message::CountLimbs => {}
            _ => {}
        }
    }
    fn view(&self) -> Element<'_, Self::Message> {
        column![
            iced::widget::row![
                // move limb right
                iced::widget::button("Right").on_press(Message::Move(Direction::Right)),
                iced::widget::button("Left").on_press(Message::Move(Direction::Left)),
                iced::widget::button("Up").on_press(Message::Move(Direction::Up)),
                iced::widget::button("Down").on_press(Message::Move(Direction::Down)),
            ],
            //iced::widget::button("Rotate Clockwise")
            //    .on_press(Message::RotateLimb(self.state.limbs.alpha + 5.0)),
            //iced::widget::button("Rotate Anti Clockwise")
            //    .on_press(Message::RotateLimb(self.state.limbs.alpha - 5.0)),
            //iced::widget::button("Get number of limbs").on_press(Message::CountLimbs),
            //]
            //.width(Length::Fill)
            //.padding(5.0),
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
struct Segment {
    start_point: Option<Point>,
    end_point: Option<Point>,
    length: f32,
    alpha: f32,
}
#[derive(Debug)]
struct Limb {
    limbs: Vec<Segment>,
}
impl Segment {
    // Calculates the end points cartesian cordiantes using alpha and length
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
    fn new(start_point: Option<Point>, length: f32, alpha: f32) -> Segment {
        let mut segment = Segment {
            start_point,
            end_point: None, // this will be calculated later with calculate_b()
            length,
            alpha: alpha.to_radians(),
        };
        segment.calculate_b();
        segment
    }
}
//fn rotate_all(&mut self, angle: f32) {
//    self.alpha += angle.to_radians();
//    self.calculate_b();
//    self.update_children();
//    if let Some(next) = &mut self.next {
//        next.rotate_all(angle);
//        next.calculate_b();
//        next.update_children();
//    }
//}
//fn update_children(&mut self) {
//    match &mut self.next {
//        Some(next) => {
//            if let Some(previous_point) = self.end_point {
//                let end_point_x: f32 = previous_point.x + (next.alpha.cos() * next.length);
//                let end_point_y: f32 = previous_point.y + (next.alpha.sin() * next.length);
//                next.start_point = Some(previous_point);
//                next.end_point = Some(Point::new(end_point_x, end_point_y));
//            }
//            next.update_children()
//        }
//        None => {}
//    }
//}
impl Limb {
    fn new(limbs: Vec<Segment>) -> Limb {
        let mut limb = Limb { limbs };
        limb.update_children();
        limb
    }
    fn render(&self, frame: &mut Frame) {
        for seg in self.limbs.iter() {
            // iterate through each point
            if let Some(start) = seg.start_point {
                // start point some ?
                if let Some(end) = seg.end_point {
                    // end point some ?
                    let spoint = Path::circle(start, 5.0);
                    let epoint = Path::circle(end, 5.0);
                    let line = Path::line(start, end);
                    frame.stroke(&line, Stroke::default().with_width(5.0));
                    frame.fill(&spoint, Color::BLACK); // adding startpoint to canvas
                    frame.fill(&epoint, Color::BLACK); // adding endpoint to canvas
                } else {
                    println!("No end point");
                }
            } else {
                println!("No start point");
            }
        }
    }
    fn move_origin(&mut self, direction: Direction) {
        // get mutable reference to first point in limbs
        let speed: f32 = 5.0;
        let first_point = self.limbs.iter_mut().next().unwrap();
        match &mut first_point.start_point {
            Some(start_point) => {
                match direction {
                    Direction::Up => start_point.y -= speed,
                    Direction::Down => start_point.y += speed,
                    Direction::Right => start_point.x += speed,
                    Direction::Left => start_point.x -= speed,
                }
                first_point.calculate_b();
                self.update_children();
            }
            None => {}
        }
    }
    fn update_children(&mut self) {
        // TODO: every segment needs to have the start point of the previous one
        // get start point of first value
        let mut iters = self.limbs.iter_mut();
        let mut previous_point = iters.next().unwrap().end_point.unwrap();
        for n in iters {
            n.start_point = Some(previous_point);
            n.calculate_b();
            previous_point = n.end_point.unwrap();
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
        self.limbs.render(&mut frame); // rendering limbs to screen
        vec![frame.into_geometry()]
    }
}
