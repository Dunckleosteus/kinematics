use iced::mouse::Cursor;
use iced::theme::Theme;
use iced::widget::canvas::{self, Frame, Geometry, Path, Program, Stroke};
use iced::widget::{column, Canvas};
use iced::{Color, Element, Length, Point, Sandbox, Settings};

#[derive(Debug, Clone)]
pub enum Message {
    MoveTarget(Point),
    Move(Direction),
    RotateLimb(f32),
    GetLength, // gets overall limb length
}
// used in move origin function
#[derive(Debug, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
// used in rotate function
pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}
pub struct Hello {
    _show_grid: bool, // TODO: add a toggle button to show grid
    state: Circle,    // <- Canvas as a field
}
impl Sandbox for Hello {
    type Message = Message;
    fn new() -> Hello {
        Hello {
            _show_grid: false,
            state: Circle {
                limbs: Limb::new(vec![
                    Segment::new(Some(Point::new(100.0, 100.0)), 100.0, 30.0),
                    Segment::new(None, 100.0, 0.0),
                    Segment::new(None, 100.0, 30.0),
                ]),
                target: None,
            },
        }
    }
    fn title(&self) -> String {
        String::from("Hello")
    }
    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Move(x) => self.state.limbs.move_origin(x),
            Message::RotateLimb(x) => self.state.limbs.rotate(x, None),
            Message::MoveTarget(x) => match &mut self.state.target {
                Some(target) => target.position = x,
                None => self.state.target = Some(Target { position: x }),
            },
            Message::GetLength => self.state.limbs.get_total_length(),
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
                iced::widget::button("Clockwise").on_press(Message::RotateLimb(5.0)),
                iced::widget::button("Counter Clockwise").on_press(Message::RotateLimb(-5.0)),
            ]
            .padding(10.0),
            iced::widget::row![Canvas::new(&self.state)
                .width(Length::Fill)
                .height(Length::Fill)]
            .padding(10.0)
        ]
        .align_items(iced::Alignment::Center)
        .into()
    }
}
// canvas
#[derive(Debug)]
struct Circle {
    limbs: Limb,
    target: Option<Target>,
}
#[derive(Debug)]
struct Target {
    position: Point,
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
    origin: Option<Point>,
    limbs: Vec<Segment>,
    length: Option<f32>,
}
impl Target {
    fn render(&self, frame: &mut Frame) {
        let circle = Path::circle(self.position, 5.0);
        frame.fill(&circle, Color::from_rgb(1., 0., 0.)); // rendering target to canvas
    }
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
impl Limb {
    fn new(limbs: Vec<Segment>) -> Limb {
        let mut limb = Limb {
            origin: None,
            limbs,
            length: None,
        };
        limb.update_children();
        limb.update_origin();
        limb.get_total_length();
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
    fn get_total_length(&mut self) {
        let length: f32 = self.limbs.iter().map(|x| x.length).sum();
        self.length = Some(length);
    }
    fn rotate(&mut self, rotation: f32, segment_id: Option<usize>) {
        let start_point: usize = segment_id.unwrap_or(0);
        let alpha = rotation.to_radians();
        for section in self.limbs.iter_mut().skip(start_point) {
            section.alpha += alpha;
            section.calculate_b();
        }
        self.update_children();
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
                self.update_origin();
            }
            None => {}
        }
    }
    fn update_origin(&mut self) {
        let first_point = self.limbs.iter_mut().next().unwrap();
        if let Some(start_point) = first_point.start_point {
            self.origin = Some(start_point.clone());
        }
    }
    fn update_children(&mut self) {
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
        event: canvas::Event,
        bounds: iced::Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        // start by checking to see if cursor is in canvas, if it is not return
        // empty message and exit update function
        let cursor_position = if let Some(position) = cursor.position_in(bounds) {
            position
        } else {
            return (iced::event::Status::Ignored, None);
        };
        match event {
            iced::widget::canvas::Event::Mouse(mouse) => match mouse {
                iced::mouse::Event::ButtonPressed(button) => match button {
                    iced::mouse::Button::Left => {
                        return (
                            canvas::event::Status::Captured,
                            Some(Message::MoveTarget(cursor_position)),
                        )
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
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
        // rendering grid -- start
        let width = bounds.width;
        let height = bounds.height;
        let grid_horizontal = 10;
        let grid_vertical = 5;
        let horizontal_delta = width / (grid_horizontal as f32);
        let vertical_delta = height / (grid_vertical as f32);
        // drawing horizontal gridlines
        for y in 0..grid_vertical {
            let start = Point::new(0.0, y as f32 * vertical_delta);
            let end = Point::new(width, y as f32 * vertical_delta);
            let line = Path::line(start, end);
            frame.stroke(&line, Stroke::default().with_width(1.0));
        }
        for x in 0..grid_horizontal {
            let start = Point::new(x as f32 * horizontal_delta, 0.0);
            let end = Point::new(x as f32 * horizontal_delta, height);
            let line = Path::line(start, end);
            frame.stroke(&line, Stroke::default().with_width(1.0));
        }
        // render canvas extent -- start
        let left = Path::line(Point::new(0.0, 0.0), Point::new(0.0, height));
        frame.stroke(&left, Stroke::default().with_width(5.0));
        let top = Path::line(Point::new(0.0, 0.0), Point::new(width, 0.0));
        frame.stroke(&top, Stroke::default().with_width(5.0));
        let right = Path::line(Point::new(width, 0.0), Point::new(width, height));
        frame.stroke(&right, Stroke::default().with_width(10.0));
        let bott = Path::line(Point::new(0.0, height), Point::new(width, height));
        frame.stroke(&bott, Stroke::default().with_width(10.0));
        // render canvas extent -- end
        if let Some(target) = &self.target {
            target.render(&mut frame);
        }
        if let Some(origin) = &self.limbs.origin {
            let shoulder = Path::circle(*origin, 10.0);
            frame.fill(&shoulder, Color::from_rgb(0.0, 1.0, 0.0));
            // displayin maximum limb reach as a circle around limb origin
            if let Some(length) = &self.limbs.length {
                let reach = Path::circle(*origin, *length);
                frame.stroke(
                    &reach,
                    Stroke::default()
                        .with_width(1.0)
                        .with_color(Color::from_rgba(0.2, 0.2, 0.2, 0.8)),
                );
            }
        }
        self.limbs.render(&mut frame); // rendering limbs to screen
        vec![frame.into_geometry()]
    }
}
