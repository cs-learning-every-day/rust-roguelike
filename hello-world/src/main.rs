extern crate rltk;
use rltk::{ GameState, Rltk, RltkBuilder};

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello RLTK World");
    }
}

fn main() {
    let context = RltkBuilder::simple80x50()
        .with_title("Rougelike Tutorial")
        .build()
        .ok()
        .unwrap();
    let gs = State {};
    let _ = rltk::main_loop(context, gs);
}
