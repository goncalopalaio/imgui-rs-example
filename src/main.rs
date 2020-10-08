use imgui::*;
use std::ops::Range;

mod support;

const HEIGHT: f32 = 100.0;
const X_OFFSET: f32 = 400.0;
const Y_OFFSET: f32 = 120.0;

struct State {
    checked: bool,
    text: ImString,
    scroll_to_end: bool,
    has_filter: bool,
    filter_curr: ImString,
}

fn clipper<F>(ui: &Ui, count: usize, mut fun: F)
where
    F: FnMut(Range<usize>),
{
    use imgui_sys::{
        ImGuiListClipper, ImGuiListClipper_Begin, ImGuiListClipper_End, ImGuiListClipper_Step,
    };

    let font_height = ui.text_line_height_with_spacing();
    let mut clipper = ImGuiListClipper {
        StartPosY: 0.0,
        ItemsHeight: -1.0,
        ItemsCount: -1,
        StepNo: 0,
        DisplayStart: 0,
        DisplayEnd: 0,
    };

    unsafe {
        ImGuiListClipper_Begin(
            &mut clipper as *mut ImGuiListClipper,
            count as std::os::raw::c_int,
            font_height as std::os::raw::c_float,
        );
    }

    while unsafe { ImGuiListClipper_Step(&mut clipper as *mut ImGuiListClipper) } {
        fun(clipper.DisplayStart as usize..clipper.DisplayEnd as usize);
    }

    unsafe {
        ImGuiListClipper_End(&mut clipper as *mut ImGuiListClipper);
    }
}

fn main() {
    println!("Hello, world!");

    let mut state = State {
        checked: false,
        text: ImString::with_capacity(128),
        scroll_to_end: false,
        has_filter: false,
        filter_curr: ImString::with_capacity(128),
    };

    let mut line_num = 0;
    let mut lines = Vec::new();

    let system = support::init(file!());
    system.main_loop(move |_, ui| {
        line_num += 1;
        lines.push(format!("Line {}", line_num));

        // Start discarding old lines after a while.
        if line_num > 1000 {
            lines.remove(0);
        }

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
                TreeNode::new(im_str!("Modals")).build(&ui, || {
                    ui.checkbox(im_str!("Is it checked (again)?"), &mut state.checked);
                });
            });

        Window::new(im_str!("Logging example"))
            .size([400.0, HEIGHT * 6.0], Condition::FirstUseEver)
            .position([100.0 + X_OFFSET, Y_OFFSET], Condition::FirstUseEver)
            .build(ui, || {
                ChildWindow::new("Options")
                    .size([0.0, 70.0])
                    .border(true)
                    .build(ui, || {
                        ui.input_text(im_str!("Filter"), &mut state.filter_curr)
                            .build();
                        ui.checkbox(im_str!("Scroll to end"), &mut state.scroll_to_end);

                        // Save state
                        state.has_filter = !state.filter_curr.to_string().is_empty();
                    });

                ChildWindow::new("Logs").border(false).build(ui, || {
                    if state.has_filter {
                        let query = state.filter_curr.to_string();
                        let result = lines
                            .iter()
                            .filter(|x| x.contains(&query))
                            .collect::<Vec<_>>();

                        clipper(ui, result.len(), |range| {
                            for i in range.start..range.end {
                                let line = &result[i];
                                ui.text(format!("{} contains {}", line, state.filter_curr));
                            }
                        });
                    } else {
                        clipper(ui, lines.len(), |range| {
                            for i in range.start..range.end {
                                let line = &lines[i];
                                ui.text(format!("-> {}", line));
                            }
                        });
                    };

                    if state.scroll_to_end {
                        if line_num > 10 {
                            unsafe {
                                imgui_sys::igSetScrollY(
                                    ui.text_line_height_with_spacing() * (line_num - 1) as f32,
                                );
                            }
                        }
                    }
                });
            });
    });
}
