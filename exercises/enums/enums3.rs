// enums3.rs
// Address all the TODOs to make the tests pass!

enum Message {
    // TODO: implement the message variant types based on their usage below
    ChangeColor(u8 ,u8, u8),
    Echo(String),
    Move { x: u8, y: u8 },
    Quit,
}

struct Point {
    x: u8,
    y: u8
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants
        match message {
            Message::ChangeColor(a, b, c) => State::change_color(self, (a, b, c)),
            Message::Quit => State::quit(self),
            Message::Echo(message) => State::echo(self, message),
            Message::Move { x, y } => State::move_position(self, Point{x, y}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State{
            quit: false,
            position: Point{ x: 0, y: 0 },
            color: (0, 0, 0)
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move{ x: 10, y: 15 });
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
    }

}
