use iced::{
    self, Element, Result,
    widget::{Column, button, row, text},
};

pub fn run() -> Result {
    iced::run("Title", update, view)
}

fn update(states: &mut States, msg: Message) {
    match msg {
        Message::Inc(idx) => {
            let state = &mut states.states[idx];
            println!("{:?}", state);
            state.age += 1;
        }
        Message::Add => {
            states.add();
        }
    }
}

fn view(states: &States) -> Element<Message> {
    let mut col = Column::with_capacity(states.states.len() + 1);
    col = col.extend(states.states.iter().map(|r| {
        button(row![text(r.id), text(r.age)].spacing(20))
            .on_press(Message::Inc(r.id))
            .into()
    }));
    col.push(button("New").on_press(Message::Add)).into()
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Inc(usize),
    Add,
}

#[derive(Debug, Default)]
struct States {
    states: Vec<State>,
}

impl States {
    fn add(&mut self) {
        self.states.push(State {
            id: self.states.len(),
            age: 0,
        })
    }
}

#[derive(Debug, Default)]
struct State {
    id: usize,
    age: i64,
}
