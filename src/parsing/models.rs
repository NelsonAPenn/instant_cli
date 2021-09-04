use std::collections::{HashSet, HashMap};

/// A struct that represents the command parsed from the list of tokens (usually
/// env::args).
pub struct ShellCommand
{
    pub executable: String,
    pub subcommands: Vec<String>,
    pub short_flags: HashMap<ShortName, u8>,
    pub short_options: HashMap<ShortName, HashSet<String>>,
    pub long_flags: HashMap<LongName, u8>,
    pub long_options: HashMap<LongName, HashSet<String>>,
    pub arguments: Vec<String>
}

pub struct ShortName(char);

pub struct LongName(String);

pub enum ShellTokenType
{
    Subcommand,
    ShortOptionGroup,
    LongOption,
    Other
}