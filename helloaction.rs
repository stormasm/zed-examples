use gpui::*;

struct HelloWorld {
    text: SharedString,
    focus: FocusHandle,
}

actions!(hw, [Draw, Hola, Quit]);

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .id("bill")
            .active(|style| style.bg(red()))
            .track_focus(&self.focus)
            // if key_context is not here it doesn't quit
            .key_context("hello_joe")
            .flex()
            .bg(rgb(0x2e7d32))
            .size(Length::Definite(Pixels(300.0).into()))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
    }
}

fn print_draw() {
    println!("Draw!");
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.on_action(|_: &Draw, _cx| print_draw());
        cx.on_action(|_: &Hola, _cx| println!("Hola!"));
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, Some("hello"))]);
        cx.bind_keys([KeyBinding::new("cmd-i", Quit, Some("hello_joe"))]);
        cx.bind_keys([KeyBinding::new("cmd-d", Draw, Some("hello_joe"))]);
        cx.bind_keys([KeyBinding::new("cmd-h", Hola, Some("hello_joe"))]);
        let bounds = Bounds::centered(None, size(px(600.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| {
                cx.new_view(|cx| HelloWorld {
                    text: "World".into(),
                    focus: cx.focus_handle(),
                })
            },
        );
    });
}
