extern crate regex;
mod models;

use std::collections::{HashSet, HashMap};
use models::{ShellCommand, ShellTokenType};
use regex::Regex;

struct ShellCommandParser
{
    subcommand_regex: Regex,
    short_flag_group_regex: Regex,
    long_option_name_regex: Regex
}

impl ShellCommandParser
{
    pub fn new() -> Self
    {
        Self
        {
            subcommand_regex:
                Regex::new("^[a-z]+(-[a-z]+)*$").unwrap(),
            short_flag_group_regex:
                Regex::new("^-[a-z]+$").unwrap(),
            long_option_name_regex:
                Regex::new("^--[a-z]+(-[a-z]+)*$").unwrap()
        }
    }

    pub fn parse(&self, tokens: Vec<String>) -> Result<ShellCommand, String>
    {
        let mut tokens_it = tokens.into_iter();

        let executable = tokens_it
            .next()
            .ok_or("No executable found in tokens list.")?;
    
        let mut subcommands = vec![];
        let mut short_flags = HashMap::new();
        let mut short_options = HashMap::new();
        let mut long_flags = HashMap::new();
        let mut long_options = HashMap::new();
        let mut arguments = vec![];
        
        let mut tokens_peekable = tokens_it.peekable();

        'subcommand: loop
        {
            if let Some(token) = tokens_peekable.peek()
            {
                if let ShellTokenType::Subcommand = self.get_token_type(
                    &token[..]
                )
                {
                    subcommands.push(tokens_peekable.next().unwrap());
                    continue 'subcommand;
                }
            }
            break;
        }
        
        'main: loop
        {
            if let Some(token) = tokens_peekable.peek()
            {
                match self.get_token_type(&token[..])
                {
                    ShellTokenType::ShortOptionGroup => {
                        
                    }
                    ShellTokenType::LongOption => {
                        let argument_name = tokens_peekable.next().unwrap();
                        let argument_value = tokens_peekable.next().ok_or(
                            format!(
                                "Expected argument value for option '{}'", argument_name
                            )
                        )?;
                    }
                    _ => {

                    }
                }
            }
            break;
        }
        
        Ok(
            ShellCommand{
                executable,
                subcommands,
                short_flags,
                short_options,
                long_flags,
                long_options,
                arguments
            }
        )
    }
    
    fn get_token_type(&self, token: &str) -> ShellTokenType
    {
        if self.subcommand_regex.is_match(token)
        {
            return ShellTokenType::Subcommand;
        }

        if self.long_option_name_regex.is_match(token)
        {
            return ShellTokenType::LongOption;
        }

        if self.short_flag_group_regex.is_match(token)
        {
            return ShellTokenType::ShortOptionGroup;
        }
        
        ShellTokenType::Other
    }
}


