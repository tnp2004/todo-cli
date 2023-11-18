use clap::ArgMatches;

pub trait ArgParser {
    fn parse_arg(&self, arg_name: &String) -> Option<&String>;
    fn parse_sub_arg(&self, action: &String, arg_name: &String) -> Option<&String>;
}

impl ArgParser for ArgMatches {
    fn parse_arg(&self, arg_name: &String) -> Option<&String> {
        match self.get_one::<String>(arg_name) {
            Some(arg_value) => Some(arg_value),
            None => None,
        }
    }

    fn parse_sub_arg(&self, action: &String, arg_name: &String) -> Option<&String> {
        match self
            .subcommand_matches(action)
            .unwrap()
            .get_one::<String>(arg_name)
        {
            Some(flag_arg_value) => Some(flag_arg_value),
            None => None,
        }
    }
}