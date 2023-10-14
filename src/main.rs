pub fn main() {
    let board = checkers::Board {
        black: 4095,
        white: 4293918720,
        kings: 1,
    };

    board.print();

    println!("{}", board.format());
}
