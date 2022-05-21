use std::rc::Rc;

use calc_engine::calculate;
use fltk::{
    app::{self},
    button::{self},
    group::{Flex, FlexType},
    prelude::*,
    text::{TextBuffer, TextDisplay},
    window::Window,
};
use fltk_theme::{color_themes, ColorTheme, SchemeType, WidgetScheme};

macro_rules! in_column {
    ($e:block) => {{
        let mut col = Flex::default_fill();
        col.set_type(FlexType::Column);

        $e;

        col.end();
    }};
}

fn execute(display: Rc<TextDisplay>) {
    let mut buf = display.buffer().unwrap();
    buf.set_text(&match calculate(&buf.text()) {
        Ok(v) => {
            v.to_string()
        }
        _ => "Error".to_string(),
    });
}

fn main() {
    let app = app::App::default();

    ColorTheme::new(color_themes::BLACK_THEME).apply();
    WidgetScheme::new(SchemeType::Fluent).apply();

    let mut wind = Window::new(0, 0, 200, 300, "Hello from rust");
    wind.make_resizable(true);

    let mut col = Flex::default_fill();
    col.set_type(FlexType::Column);

    let buffer = TextBuffer::default();
    let mut display = TextDisplay::default_fill();
    display.set_buffer(buffer);

    let mut row = Flex::default_fill();
    row.set_type(FlexType::Row);

    let display = Rc::new(display);

    in_column!({
        let label1 = "7";
        let label2 = "4";
        let label3 = "1";

        let mut btn1 = button::Button::default()
            .with_size(50, 50)
            .center_of_parent()
            .with_label(label1);
        let mut btn2 = button::Button::default()
            .with_size(50, 50)
            .center_of_parent()
            .with_label(label2);
        let mut btn3 = button::Button::default()
            .with_size(50, 50)
            .center_of_parent()
            .with_label(label3);

        btn1.set_callback({
            let display = display.clone();
            move |_| {
                let mut buf = display.buffer().unwrap();

                if buf.text() == "Error" {
                    buf.set_text("");
                }

                buf.append(label1);
            }
        });

        btn2.set_callback({
            let display = display.clone();
            move |_| {
                let mut buf = display.buffer().unwrap();

                if buf.text() == "Error" {
                    buf.set_text("");
                }

                buf.append(label2);
            }
        });

        btn3.set_callback({
            let display = display.clone();
            move |_| {
                let mut buf = display.buffer().unwrap();

                if buf.text() == "Error" {
                    buf.set_text("");
                }

                buf.append(label3);
            }
        });
    });

    in_column!({
        let label1 = "8";
        let label2 = "5";
        let label3 = "2";

        let mut btn1 = button::Button::default()
            .with_size(50, 50)
            .center_of_parent()
            .with_label(label1);
        let mut btn2 = button::Button::default()
            .with_size(50, 50)
            .center_of_parent()
            .with_label(label2);
        let mut btn3 = button::Button::default()
            .with_size(50, 50)
            .center_of_parent()
            .with_label(label3);

        btn1.set_callback({
            let display = display.clone();
            move |_| {
                let mut buf = display.buffer().unwrap();

                if buf.text() == "Error" {
                    buf.set_text("");
                }

                buf.append(label1);
            }
        });

        btn2.set_callback({
            let display = display.clone();
            move |_| {
                let mut buf = display.buffer().unwrap();

                if buf.text() == "Error" {
                    buf.set_text("");
                }

                buf.append(label2);
            }
        });

        btn3.set_callback({
            let display = display.clone();
            move |_| {
                let mut buf = display.buffer().unwrap();

                if buf.text() == "Error" {
                    buf.set_text("");
                }

                buf.append(label3);
            }
        });
    });

    in_column!({
        let label1 = "9";
        let label2 = "6";
        let label3 = "3";

        let mut btn1 = button::Button::default()
            .with_size(50, 50)
            .center_of_parent()
            .with_label(label1);
        let mut btn2 = button::Button::default()
            .with_size(50, 50)
            .center_of_parent()
            .with_label(label2);
        let mut btn3 = button::Button::default()
            .with_size(50, 50)
            .center_of_parent()
            .with_label(label3);

        btn1.set_callback({
            let display = display.clone();
            move |_| {
                let mut buf = display.buffer().unwrap();

                if buf.text() == "Error" {
                    buf.set_text("");
                }

                buf.append(label1);
            }
        });

        btn2.set_callback({
            let display = display.clone();
            move |_| {
                let mut buf = display.buffer().unwrap();

                if buf.text() == "Error" {
                    buf.set_text("");
                }

                buf.append(label2);
            }
        });

        btn3.set_callback({
            let display = display.clone();
            move |_| {
                let mut buf = display.buffer().unwrap();

                if buf.text() == "Error" {
                    buf.set_text("");
                }

                buf.append(label3);
            }
        });
    });

    in_column!({
        let label1 = "CLR";
        let label2 = ".";
        let label3 = "0";

        let mut btn1 = button::Button::default()
            .with_size(50, 50)
            .center_of_parent()
            .with_label(label1);
        let mut btn2 = button::Button::default()
            .with_size(50, 50)
            .center_of_parent()
            .with_label(label2);
        let mut btn3 = button::Button::default()
            .with_size(50, 50)
            .center_of_parent()
            .with_label(label3);

        btn1.set_callback({
            let display = display.clone();
            move |_| {
                display.buffer().unwrap().set_text("");
            }
        });

        btn2.set_callback({
            let display = display.clone();
            move |_| {
                let mut buf = display.buffer().unwrap();
                let text = buf.text();

                if text == "Error" {
                    buf.set_text("");
                }

                if text.chars().last().unwrap_or(' ') != '.' {
                    buf.append(label2);
                }
            }
        });

        btn3.set_callback({
            let display = display.clone();
            move |_| {
                let mut buf = display.buffer().unwrap();

                if buf.text() == "Error" {
                    buf.set_text("");
                }

                buf.append(label3);
            }
        });
    });

    in_column!({
        let label1 = "<";
        let label2 = "+";
        let label3 = "-";

        let mut btn1 = button::Button::default()
            .with_size(50, 50)
            .center_of_parent()
            .with_label(label1);
        let mut btn2 = button::Button::default()
            .with_size(50, 50)
            .center_of_parent()
            .with_label(label2);
        let mut btn3 = button::Button::default()
            .with_size(50, 50)
            .center_of_parent()
            .with_label(label3);

        btn1.set_callback({
            let display = display.clone();
            move |_| {
                let mut buf = display.buffer().unwrap();

                if buf.text() == "Error" {
                    buf.set_text("");
                }

                let len = buf.length();
                buf.remove(len - 1, len);
            }
        });

        btn2.set_callback({
            let display = display.clone();
            move |_| {
                let mut buf = display.buffer().unwrap();

                if buf.text() == "Error" {
                    buf.set_text("");
                }

                buf.append(label2);
            }
        });

        btn3.set_callback({
            let display = display.clone();
            move |_| {
                let mut buf = display.buffer().unwrap();

                if buf.text() == "Error" {
                    buf.set_text("");
                }

                buf.append(label3);
            }
        });
    });

    in_column!({
        let label1 = "*";
        let label2 = "/";
        let label3 = "=";

        let mut btn1 = button::Button::default()
            .with_size(50, 50)
            .center_of_parent()
            .with_label(label1);
        let mut btn2 = button::Button::default()
            .with_size(50, 50)
            .center_of_parent()
            .with_label(label2);
        let mut btn3 = button::Button::default()
            .with_size(50, 50)
            .center_of_parent()
            .with_label(label3);

        btn1.set_callback({
            let display = display.clone();
            move |_| {
                let mut buf = display.buffer().unwrap();

                if buf.text() == "Error" {
                    buf.set_text("");
                }

                display.buffer().unwrap().append(label1);
            }
        });

        btn2.set_callback({
            let display = display.clone();
            move |_| {
                let mut buf = display.buffer().unwrap();

                if buf.text() == "Error" {
                    buf.set_text("");
                }

                display.buffer().unwrap().append(label2);
            }
        });

        btn3.set_callback(move |_| execute(display.clone()));
    });

    row.end();
    col.end();
    wind.end();
    wind.show();
    app.run().unwrap();
}
