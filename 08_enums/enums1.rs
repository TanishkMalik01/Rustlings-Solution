#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize = 232,
    Move = 43,
    Echo = 12,
    ChangeColor = 54,
    Quit = 32,
    Exti,
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
