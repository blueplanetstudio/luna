#![allow(unused, dead_code)]

//! # Luna: A software design tool without compromises.
//!
//! Luna is local, files on disk first. Own your own data,
//! collaborate, design on the canvas or write code.
//!
//! It's not a design tool, or a code editor, it's a tool
//! for designing software:
//!
//! That means not just pixels, but representative screens and flows
//! using an abstractionless design and editing experience.

use gpui::{
    actions, div, hsla, point, prelude::*, px, rgba, App, AppContext, Application, Entity,
    FocusHandle, Focusable, KeyBinding, Keystroke, Menu, MenuItem, MouseButton, MouseUpEvent, Rgba,
    TitlebarOptions, Window, WindowOptions,
};
use input::text_input::TextInput;
mod geometry;
mod input;

actions!(luna, [Quit]);

struct Luna {
    // The main canvas where elements are rendered and manipulated
    // active_canvas: Entity<LunaCanvas>,
    /// Focus handle for keyboard event routing
    focus_handle: FocusHandle,
    text_input: Entity<TextInput>,
}

impl Luna {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let text_input = cx.new(|cx| TextInput {
            focus_handle: cx.focus_handle(),
            content: "".into(),
            placeholder: "Type here...".into(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            is_selecting: false,
        });

        Self {
            focus_handle: cx.focus_handle(),
            text_input,
        }
    }

    fn focus_handle(&self) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Focusable for Luna {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Luna {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle())
            .items_center()
            .justify_center()
            .id("scene-graph")
            .key_context("Luna")
            .track_focus(&self.focus_handle())
            .text_xs()
            .font_family("Berkeley Mono")
            .flex()
            .flex_col()
            .relative()
            .bg(hsla(0.0, 0.0, 0.0, 1.0))
            .size_full()
            .text_color(hsla(0.0, 1.0, 1.0, 1.0))
            .child(self.text_input.clone())
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.on_action(quit);
        cx.set_menus(vec![Menu {
            name: "Luna".into(),
            items: vec![MenuItem::action("Quit", Quit)],
        }]);

        cx.bind_keys([
            KeyBinding::new("backspace", input::Backspace, None),
            KeyBinding::new("delete", input::Delete, None),
            KeyBinding::new("left", input::Left, None),
            KeyBinding::new("right", input::Right, None),
            KeyBinding::new("shift-left", input::SelectLeft, None),
            KeyBinding::new("shift-right", input::SelectRight, None),
            KeyBinding::new("cmd-a", input::SelectAll, None),
            KeyBinding::new("cmd-v", input::Paste, None),
            KeyBinding::new("cmd-c", input::Copy, None),
            KeyBinding::new("cmd-x", input::Cut, None),
            KeyBinding::new("home", input::Home, None),
            KeyBinding::new("end", input::End, None),
            KeyBinding::new("ctrl-cmd-space", input::ShowCharacterPalette, None),
        ]);

        let window = cx
            .open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Luna".into()),
                        traffic_light_position: Some(point(px(8.0), px(8.0))),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |window, cx| cx.new(|cx| Luna::new(window, cx)),
            )
            .unwrap();

        cx.on_keyboard_layout_change({
            move |cx| {
                window.update(cx, |_, _, cx| cx.notify()).ok();
            }
        })
        .detach();

        window
            .update(cx, |view, window, cx| {
                window.focus(&view.focus_handle());
                cx.activate(true);
            })
            .unwrap();

        // window
        //     .update(cx, |view, window, cx| {
        //         window.focus(&view.focus_handle());
        //         cx.activate(true);
        //     })
        //     .unwrap();
    });
}

fn quit(_: &Quit, cx: &mut App) {
    cx.quit();
}
