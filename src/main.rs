mod app;
mod cli;

use app::Application;
use cli::runner::run;


fn main() {
    let app = Application::new();
    run(&app);
}
