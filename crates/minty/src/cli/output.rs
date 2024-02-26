mod about;
mod tag;
mod time;
mod view;

pub use about::About;

use serde::Serialize;
use serde_json as json;
use std::io::{stderr, stdout, IsTerminal, Result, Write};

pub trait HumanReadable {
    fn human_readable<W: Write>(&self, write: W) -> Result<()>;
}

#[derive(Clone, Copy, Debug)]
pub struct Output {
    pub human_readable: bool,
    pub json: bool,
}

pub trait Print {
    fn print(&self, output: Output) -> Result<()>;
}

impl<T> Print for T
where
    T: Sized + Serialize + HumanReadable,
{
    fn print(&self, output: Output) -> Result<()> {
        let print_json = || println!("{}", json::to_string(self).unwrap());

        if output.json {
            print_json();

            if output.human_readable {
                self.human_readable(stderr())?;
            }
        } else if output.human_readable || stdout().is_terminal() {
            self.human_readable(stdout())?;
        } else {
            print_json();
        }

        Ok(())
    }
}
