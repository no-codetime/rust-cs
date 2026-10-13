#![windows_subsystem = "windows"]

//! 最小 WinUI 3 窗口示范：Rust(windows-reactor) 驱动原生 WinUI 3 控件。
//!
//! 组件循环：原生事件 → 类型化消息 → update 改状态 → view 重新描述界面。

use windows_reactor::*;

#[derive(Clone)]
enum Message {
    Input(String),
    Echo,
    Clear,
}

struct WindowDemo {
    input: String,
    greeting: String,
    show_hint: bool,
}

impl Component for WindowDemo {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            input: String::new(),
            greeting: String::new(),
            show_hint: false,
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Input(value) => self.input = value,
            Message::Echo => {
                let text = self.input.trim();
                if text.is_empty() {
                    self.greeting.clear();
                    self.show_hint = true;
                } else {
                    self.greeting = format!("你好，{text}！");
                    self.show_hint = false;
                }
            }
            Message::Clear => {
                self.input.clear();
                self.greeting.clear();
                self.show_hint = false;
            }
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("WinUI 3 最小窗口示范");
        context.window_visuals(
            WindowVisuals::new()
                .backdrop(WindowBackdrop::Mica)
                .client_size(760.0, 480.0),
        );

        StackPanel::new()
            .margin(28.0)
            .spacing(14.0)
            .children((
                TextBlock::new().text("窗口骨架").font_size(24.0),
                TextBlock::new()
                    .text("Rust(windows-reactor) → 原生 WinUI 3：受控 TextBox、Button 消息、InfoBar 提示。")
                    .text_wrapping(TextWrapping::Wrap),
                TextBox::new()
                    .text(self.input.as_str())
                    .placeholder_text("例如：杨锦江")
                    .on_text_changed(context.callback(Message::Input)),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(12.0)
                    .children((
                        Button::new()
                            .style(ButtonStyle::Accent)
                            .on_click(context.message(Message::Echo))
                            .content("回显"),
                        Button::new()
                            .on_click(context.message(Message::Clear))
                            .content("清空"),
                    )),
                InfoBar::new()
                    .is_open(self.show_hint)
                    .severity(InfoBarSeverity::Warning)
                    .title("提示")
                    .message("先在输入框里写点内容再点回显。"),
                TextBlock::new().text(self.greeting.as_str()).font_size(20.0),
            ))
    }
}

fn main() {
    App::run_component::<WindowDemo>(()).unwrap();
}
