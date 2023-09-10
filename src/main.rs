use window::run;

mod window;
mod renderer;


fn main() {
    pollster::block_on(run())
}
