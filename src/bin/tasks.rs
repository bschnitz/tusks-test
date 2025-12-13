use tusks::tusks;

#[tusks(root)]
#[command(
    about = "Task management binary",
    long_about = "A binary for managing tasks with git and docker submodules",
    version = "1.0.0",
    author = "Task Manager"
)]
pub mod tasks {
    use clap::{CommandFactory, Parser};

    #[command(name = "t", allow_external_subcommands = true)]
    pub fn tasks_mode (
        #[arg(trailing_var_arg = true)]
        args: Vec<String>
    ) {
        let command = __internal_tusks_module::cli::Cli::command();
        if args.len() == 0 {
            let task_list = tusks_tasks::task_list::models::TaskList::from_command(
                &command,
                ".".to_string(),
                5,
                20 
            );
            task_list.to_list().print(&tusks_tasks::list::models::RenderConfig::default());
        }
        else {
            let cli = __internal_tusks_module::cli::Cli::parse_from(
                std::iter::once("tasks").chain(args.iter().map(String::as_str))
            );
            __internal_tusks_module::handle_matches(&cli);
        }
    }

    /// Main task function
    #[command(about = "Execute main task")]
    #[default]
    pub fn main_task() {
        println!("Executing main task in tasks binary");
    }

    /// Git submodule
    #[command(about = "Git operations")]
    pub mod git {
        /// Clone repository
        #[command(about = "Clone a git repository")]
        pub fn clone(url: String, path: Option<String>) {
            println!("Cloning {} to {:?}", url, path.unwrap_or("current directory".to_string()));
        }

        /// Commit changes
        #[command(about = "Commit changes")]
        pub fn commit(message: String) {
            println!("Committing with message: {}", message);
        }
    }

    /// Docker submodule
    #[command(about = "Docker operations")]
    pub mod docker {
        /// Build image
        #[command(about = "Build Docker image")]
        pub fn build(context: String, tag: Option<String>) {
            println!("Building Docker image from {} with tag {:?}", context, tag);
        }

        /// Run container
        #[command(about = "Run Docker container")]
        pub fn run(image: String, args: Vec<String>) {
            println!("Running container {} with args: {:?}", image, args);
        }
    }
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(tasks::exec_cli().unwrap_or(0) as u8)
}
