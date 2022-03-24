extern crate hazel;
use hazel::Application;


struct Sandbox;
impl hazel::Application for Sandbox {}


fn main() {
    let sandbox = Box::new(Sandbox {});
    (*sandbox).run();
}
