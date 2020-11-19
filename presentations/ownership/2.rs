#[derive(Debug)]
struct Dot {
    x: i32,
    y: i32
}

fn main() {
    let dot = Dot { x: 1, y: 2 };
    pacman(dot);
    pacman(dot); // <1>
}

fn pacman(dot: Dot) {
    println!("Eating {:?}", dot);
}
