use iced::mouse::Cursor;
use iced::widget::Canvas;
use iced::widget::canvas::Renderer;
use iced::{Element, Sandbox, Settings, Color};
use iced::widget::{Text, canvas::{self, Program, Geometry, Fill, Frame, Path}};
use iced::theme::Theme; 

pub fn main()-> iced::Result {
    Hello::run(Settings::default())
}
pub struct Hello; 
impl Sandbox for Hello {
    type Message = ();
    fn new()->Hello{
        Hello
    }
    fn title(&self)->String{
        String::from("Hello")
    }
    fn update(&mut self, _message: Self::Message){

    }
    fn view(&self) -> Element<'_, Self::Message> {
        Canvas::new(Circle{radius: 50.0}).into()
    }
}
// canvas
#[derive(Debug)]
struct Circle {
    radius: f32,
}
impl Program<()> for Circle {
    type State = ();
    fn draw(
            &self, 
            _state: &Self::State,
            renderer: &iced::Renderer, 
            _theme: &Theme, 
            bounds: iced::Rectangle, 
            _cursor: Cursor
        ) -> Vec<Geometry>{
        let mut frame = Frame::new(renderer, bounds.size());
        let circle = Path::circle(frame.center(), self.radius); 
        frame.fill(&circle, Color::BLACK); 
        vec![frame.into_geometry()]
    }
}
