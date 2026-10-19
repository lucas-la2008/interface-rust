use interface::{
    app::App, elements::button::Button
};

fn main() {
    let mut main = App::new("Ola", 200, 200);
    main.config();
    let button = main.create_element::<Button>();
    button.set_position(0.0, 0.0);
    button.set_size(100.0, 100.0);
    button.set_color(255.0, 0.0, 0.0, 1.0);
    button.set_visibility(true);
    main.run();
}
