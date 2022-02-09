use gtk::prelude::GtkWindowExt;
use super::main_window::MainWindow;
use gtk::Application;
use gtk::prelude::ApplicationExt;
use gtk::prelude::ApplicationExtManual;

pub struct App {
    application: Application
}

impl App {
    fn build_ui(app: &Application) {
        let window = MainWindow::new(app);

        gtk_layer_shell::init_for_window(&window);
        gtk_layer_shell::set_keyboard_interactivity(&window, true);
        gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Overlay);

        window.present();
    }
    pub fn new(app_id: &str) -> App {
        let app = Application::builder()
            .application_id(app_id)
            .build();

        app.connect_activate(Self::build_ui);

        App { application: app }
    }
    pub fn run(&self) -> i32 {
        self.application.run()
    }
}