use window::run;

mod renderer;
mod window;

fn main() {
    pollster::block_on(run());
}
