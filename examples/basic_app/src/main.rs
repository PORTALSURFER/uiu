use iui::Application;

fn main() {
    pretty_env_logger::init();
    Application::new().unwrap().run();
}
