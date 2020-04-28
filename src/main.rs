use anyhow::Result;
use clap;
use clap::Clap;
use log;
use mobr::{cmd, config, emoji_logger, git, session, session::Store, timer};

#[derive(Clap)]
#[clap(version = "1.0", author = "Paul")]
struct Opts {
    // #[clap(long = "dry-run")]
    // dry_run: bool,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    /// Get current status
    #[clap(name = "status")]
    Status,

    /// Clean up mob related stuff from this repo
    #[clap(name = "clean")]
    Clean,

    /// Start mob session
    #[clap(name = "start")]
    Start(cmd::StartOpts),

    /// Finish turn and sync repo
    #[clap(name = "next")]
    Next(cmd::NextOpts),

    /// Stop session
    #[clap(name = "done")]
    Done,
}

fn main() -> Result<()> {
    emoji_logger::init("debug");
    let opts: Opts = Opts::parse();

    let config = config::load()?;

    let timer = timer::ConsoleTimer::new(config.commands());
    let git = git::GitCommand::new(None, config.remote.clone())?;
    let store = session::SessionStore::new(&git);

    log::trace!("Running command {:?}", opts.subcmd);

    match opts.subcmd {
        SubCommand::Start(opts) => cmd::Start::new(&git, &store, &timer, opts, config).run()?,
        SubCommand::Next(opts) => cmd::Next::new(&git, &store, &timer, opts, config).run()?,
        SubCommand::Done => cmd::Done::new(&git, &store, config).run()?,
        SubCommand::Clean => store.clean()?,
        SubCommand::Status => {
            let session = store.load()?;
            println!("{:#?}", session);
        }
    };
    Ok(())
}
