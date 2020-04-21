use cursive::Cursive;

pub fn setup() -> Cursive {
    let mut console = Cursive::default();
    console.run();
    console
}
