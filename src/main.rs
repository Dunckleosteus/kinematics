use iced::mouse::Cursor;
use iced::widget::Canvas;
use iced::{Element, Sandbox, Settings, Color, Length, Point};
use iced::widget::canvas::{self, Program, Geometry, Frame, Path};
use iced::theme::Theme; 

#[derive(Debug)]
pub enum Message{
    MouseMoved(Point),
}

pub fn main()-> iced::Result {
    Hello::run(Settings::default())
}
pub struct Hello{
    state: Circle, // <- Canvas as a field
}
impl Sandbox for Hello {
    type Message = Message;
    fn new()->Hello{
        Hello{state: Circle{radius: 50.0, position: Point{x: 0.0, y: 0.0}}}
    }
    fn title(&self)->String{
        String::from("Hello")
    }
    fn update(&mut self, message: Self::Message){
        match message {
            Message::MouseMoved(x)=>{
                self.state.position = x
            }
        }
    }
    fn view(&self) -> Element<'_, Self::Message> {
        Canvas::new(&self.state).width(Length::Fill).height(Length::Fill).into()
    }
}
// canvas
#[derive(Debug)]
struct Circle {
    radius: f32,
    position: Point
}
impl Program<Message> for Circle {
    type State = ();
    fn update(
            &self,
            _state: &mut Self::State,
            _event: canvas::Event,
            bounds: iced::Rectangle,
            cursor: iced::mouse::Cursor,
        ) -> (canvas::event::Status, Option<Message>) {
        if let Some(position) = cursor.position_in(bounds){
            (canvas::event::Status::Captured, Some(Message::MouseMoved(position)))
        }else{
            (canvas::event::Status::Ignored, None)
        }
    }
    fn draw(
            &self, 
            _state: &Self::State,
            renderer: &iced::Renderer, 
            _theme: &Theme, 
            bounds: iced::Rectangle, 
            _cursor: Cursor
        ) -> Vec<Geometry>{
        let mut frame = Frame::new(renderer, bounds.size());
            let circle = Path::circle(self.position, self.radius); 
            frame.fill(&circle, Color::BLACK); 
        vec![frame.into_geometry()]
    }
}
