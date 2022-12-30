use self::board::Board;

extern crate libc;

mod board;

//cbindgen --config cbindgen.toml --crate connect4 --output target/release/connect4.h

#[repr(C)]
struct Connect4 {
    board: Board,
}

impl Connect4 {

    #[no_mangle]
    pub extern "C" fn connect4_make() -> Self {
        Self {
            board: Board::new()
        }
    }

    #[no_mangle]
    pub extern "C" fn connect4_print(&mut self) {
        println!("{}", self.board);
    }

    #[no_mangle]
    pub extern "C" fn connect4_place(&mut self, col: i32) {
        self.board.place(col as usize);
    }
    
}
