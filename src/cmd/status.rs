use crate::{config::Config, session};
use anyhow::Result;
use clap::{self, Clap};
use console::style;
use session::State;

#[derive(Clap, Debug)]
pub struct StatusOpts {
    /// Show raw status
    #[clap(short, long)]
    raw: bool,
}

pub struct Status<'a> {
    store: &'a dyn session::Store,
    config: Config,
    opts: StatusOpts,
}

impl<'a> Status<'a> {
    pub fn new(opts: StatusOpts, store: &'a impl session::Store, config: Config) -> Status<'a> {
        Self {
            opts,
            store,
            config,
        }
    }

    pub fn run(&self) -> Result<()> {
        let session = self.store.load()?;

        if self.opts.raw {
            println!("{:#?}", session);
            return Ok(());
        }

        self.print_status(&session);
        self.print_drivers(&session);

        Ok(())
    }

    fn print_status(&self, session: &session::Session) {
        let me = self.config.name.clone();
        match &session.state {
            State::Stopped => {
                let help = "Run 'mob start' to start a new session";
                println!("✋ {}", style("Stopped").red());
                println!("   {}", style(help).cyan());
            }
            State::Working { driver } => {
                let driver = if driver == &me {
                    "You are".to_string()
                } else {
                    format!("{} is", driver)
                };
                println!("🚗 {} {}", driver, style("driving").green(),);
                println!("   {}", style("Run 'mob next' when finished").cyan());
                self.print_branches(&session.branches);
            }
            State::WaitingForNext { next } => {
                let next = match next {
                    Some(driver) if driver == &me => "You",
                    Some(ref driver) => driver,
                    None => "Anyone",
                };

                println!(
                    "💤 {} for {} to run 'mob start'",
                    style("Waiting").blue(),
                    next
                );
                self.print_branches(&session.branches);
            }
        }
    }

    fn print_branches(&self, branches: &session::Branches) {
        println!(
            "\n🚚 working on {} with parent {}",
            style(&branches.branch).red().bold(),
            style(&branches.base_branch).cyan().bold(),
        )
    }

    fn print_drivers(&self, session: &session::Session) {
        let drivers = session.drivers.all();
        if drivers.is_empty() {
            return;
        }

        let current = match &session.state {
            State::Working { driver } => Some(driver),
            State::WaitingForNext {
                next: Some(next), ..
            } => Some(next),
            _ => None,
        };

        println!("\n👯 Drivers:");
        for driver in session.drivers.all() {
            let prefix = match current {
                Some(name) if name == &driver => "›",
                _ => " ",
            };

            println!(" {} {}", style(prefix).red(), driver);
        }
    }
}
