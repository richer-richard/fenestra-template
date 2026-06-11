//! {{project-name}}: a fenestra app. `cargo run` opens the window;
//! `cargo test` verifies the UI headlessly (no display server needed).

use fenestra::prelude::*;

pub struct App_ {
    pub count: i64,
    pub name: String,
    pub dark: bool,
}

#[derive(Clone)]
pub enum Msg {
    Inc,
    Dec,
    Name(String),
    Dark(bool),
}

impl Default for App_ {
    fn default() -> Self {
        Self {
            count: 0,
            name: String::new(),
            dark: false,
        }
    }
}

impl App for App_ {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Inc => self.count += 1,
            Msg::Dec => self.count -= 1,
            Msg::Name(s) => self.name = s,
            Msg::Dark(d) => self.dark = d,
        }
    }

    fn theme(&self) -> Theme {
        if self.dark { Theme::dark() } else { Theme::light() }
    }

    fn view(&self) -> Element<Msg> {
        let theme = self.theme();
        col()
            .w_full()
            .h_full()
            .bg(theme.bg)
            .items_center()
            .justify_center()
            .gap(SP4)
            .children([
                text("{{project-name}}").size(TextSize::Lg).weight(Weight::Semibold),
                text(self.count.to_string())
                    .size(TextSize::Xl2)
                    .weight(Weight::Semibold),
                row().gap(SP3).children([
                    button("−").variant(ButtonVariant::Secondary).on_click(Msg::Dec),
                    button("+").on_click(Msg::Inc),
                ]),
                text_input(&self.name)
                    .placeholder("Your name…")
                    .on_input(Msg::Name)
                    .id("name")
                    .into(),
                switch(self.dark)
                    .label("Dark mode")
                    .on_toggle(Msg::Dark(!self.dark))
                    .id("dark")
                    .into(),
            ])
    }
}

fn main() {
    fenestra::run(
        App_::default(),
        WindowOptions::titled("{{project-name}}").with_size(480.0, 420.0),
    )
}
