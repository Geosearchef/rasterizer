mod util;
mod image;

use crate::image::DynamicImage;
use chrono::{DateTime, Local};
use iced::event::Status;
use iced::mouse::Cursor;
use iced::widget::canvas::{Event, Geometry};
use iced::widget::{canvas, Canvas};
use iced::{time, Element, Fill, Rectangle, Size, Subscription, Theme};
use std::ops::Deref;
use std::time::Duration;

const WINDOW_TITLE: &str = "Render";
const WINDOW_SIZE: Size = Size::new(600f32, 600f32);
const TARGET_FRAME_RATE: u64 = 60;
const TARGET_FRAME_TIME_MILLIS: u64 = 1_000u64 / TARGET_FRAME_RATE;

fn main() {
    tracing_subscriber::fmt::init();

    match iced::application(WINDOW_TITLE, Application::update, Application::view)
        .subscription(Application::subscription)
        .window_size(WINDOW_SIZE)
        .centered()
        .antialiasing(true)
        .run() {
            Ok(_) => {},
            Err(e) => println!("Couldn't create application wind")
        };
}

struct Application {
    frame_buffer: DynamicImage,
    last_update: i64
} // the iced state
impl Application {
    fn new() -> Self {
        Self {
            frame_buffer: DynamicImage::new(WINDOW_SIZE.width as u32, WINDOW_SIZE.height as u32),
            last_update: Local::now().timestamp_millis()
        }
    }

    fn subscription(&self) -> Subscription<Message> { // produces a message without time every frame
        time::every(Duration::from_millis(TARGET_FRAME_TIME_MILLIS))
            .map(|_| Message::Tick(Local::now()))
    }

    fn update(&mut self, message: Message) { // message is action from view (=render)
        match message {
            Message::Tick(time) => {
                let delta = time.timestamp_millis() - self.last_update;
                self.last_update += delta;

                // This is expensive
                let start = Local::now().timestamp_millis();
                for y in 0..self.frame_buffer.height {
                    for x in 0..self.frame_buffer.width {
                        self.frame_buffer.set_rgba(x, y, (0u8, (time.timestamp_millis() / 5 % 255) as u8, 0u8, 255u8));
                    }
                }
                println!("Took {} ms", Local::now().timestamp_millis() - start);
            }
        };


    }

    fn view(&self) -> Element<Message> { // returns message based on button press, ...
        Canvas::new(CanvasRenderer::new(self)).width(Fill).height(Fill).into()
    }
}

impl Default for Application { // state auto created by iced
    fn default() -> Self { Application::new() }
}

struct CanvasRenderer<'a> {
    application: &'a Application
    // image: Image
}
impl<'a> canvas::Program<Message> for CanvasRenderer<'a> {
    type State = ();

    fn update(&self, _state: &mut Self::State, _event: Event, _bounds: Rectangle, _cursor: Cursor) -> (Status, Option<Message>) {
        (Status::Captured, None)
    }

    fn draw(&self, _: &Self::State, renderer: &iced::Renderer, theme: &Theme, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry<iced::Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        frame.draw_image(bounds, self.application.frame_buffer.to_iced_image()); // clones the frame buffer
        println!("{:?}", self.application.frame_buffer.get_rgba(10, 10));

        vec![frame.into_geometry()]
    }


}
impl<'a> CanvasRenderer<'a> {
    fn new(application: &'a Application) -> Self {
        Self {
            application
            // image: load_image("logo.png")
        }
    }
}

#[derive(Debug,Clone)]
enum Message {
    Tick(DateTime<Local>)
}
