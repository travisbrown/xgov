use cli_helpers::prelude::*;
use std::{collections::BTreeSet, path::PathBuf};

mod model;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("CLI argument reading error")]
    Args(#[from] cli_helpers::Error),
    #[error("CSV error")]
    Csv(#[from] csv::Error),
    #[error("JSON error")]
    Json(#[from] serde_json::Error),
}

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    opts.verbose.init_logging()?;

    match opts.command {
        Command::UpdateCommunityNotes {
            accounts,
            info,
            notes,
        } => {
            let ids = model::account_ids(accounts)?
                .into_iter()
                .collect::<BTreeSet<_>>();

            let mut paths = std::fs::read_dir(notes)?
                .map(|result| {
                    let entry = result?;

                    Ok(entry.path())
                })
                .collect::<Result<Vec<_>, Error>>()?;

            paths.sort();

            let mut note_entries = csv::ReaderBuilder::new()
                .from_path(&info)?
                .deserialize()
                .collect::<Result<Vec<model::NoteEntry>, _>>()?;

            for path in paths {
                let mut reader = csv::ReaderBuilder::new().from_path(path)?;

                for result in reader.deserialize::<model::NoteEntry>() {
                    let note_entry = result?;

                    if let Some(user_id) = note_entry.user_id
                        && ids.contains(&user_id) {
                            note_entries.push(note_entry);
                        }
                }
            }

            note_entries.sort_by_key(|note_entry| (note_entry.user_id, note_entry.note_id));
            note_entries.dedup();

            let mut writer = csv::WriterBuilder::new().from_path(info)?;

            for note_entry in note_entries {
                writer.serialize(note_entry)?;
            }

            writer.flush()?;
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[clap(name = "xgov", version, author)]
struct Opts {
    #[clap(flatten)]
    verbose: Verbosity,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    UpdateCommunityNotes {
        #[clap(long, default_value = "accounts.csv")]
        accounts: PathBuf,
        #[clap(long, default_value = "community-notes.csv")]
        info: PathBuf,
        #[clap(long)]
        notes: PathBuf,
    },
}
