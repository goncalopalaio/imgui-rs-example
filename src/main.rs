use imgui::*;

mod support;

const HEIGHT: f32 = 100.0;
const Y_OFFSET: f32 = 120.0;

struct State {
    checked: bool,
    text: ImString,
}

fn main() {
    println!("Hello, world!");

    let mut state = State {
        checked: false,
        text: ImString::with_capacity(128),
    };


    let system = support::init(file!());
    system.main_loop(move |_, ui| {
        Window::new(im_str!("Hello world"))
            .size([300.0, HEIGHT], Condition::FirstUseEver)
            .position([100.0, Y_OFFSET], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("こんにちは世界！"));
                ui.text(im_str!("This...is...imgui-rs!"));
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });

        Window::new(im_str!("Tabs"))
            .size([300.0, HEIGHT], Condition::FirstUseEver)
            .position([100.0, Y_OFFSET * 2.0], Condition::FirstUseEver)
            .build(ui, || {
                TabBar::new(im_str!("basictabbar")).build(&ui, || {
                    TabItem::new(im_str!("Avocado")).build(&ui, || {
                        ui.text(im_str!("This is the Avocado tab!"));
                        ui.text(im_str!("blah blah blah blah blah"));
                    });
                    TabItem::new(im_str!("Broccoli")).build(&ui, || {
                        ui.text(im_str!("This is the Broccoli tab!"));
                        ui.text(im_str!("blah blah blah blah blah"));
                    });
                    TabItem::new(im_str!("Cucumber")).build(&ui, || {
                        ui.text(im_str!("This is the Cucumber tab!"));
                        ui.text(im_str!("blah blah blah blah blah"));
                    });
                });
            });

        Window::new(im_str!("Collapsing Header"))
            .size([300.0, HEIGHT], Condition::FirstUseEver)
            .position([100.0, Y_OFFSET * 3.0], Condition::FirstUseEver)
            .build(ui, || {
                if CollapsingHeader::new(im_str!("Window options")).build(&ui) {
                    ui.checkbox(im_str!("Is it checked?"), &mut state.checked);
                    ui.input_text_multiline(im_str!("multiline"), &mut state.text, [300., 100.])
                        .build();
                }
            });

        Window::new(im_str!("Inputs"))
            .always_auto_resize(true)
            .position([100.0, Y_OFFSET * 4.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.checkbox(im_str!("Is it checked?"), &mut state.checked);
                ui.input_text_multiline(im_str!("multiline"), &mut state.text, [300., 100.])
                    .build();
            });
    });
}
