use std::env;
use std::str::FromStr;

/// Rustified version of Quake's parameter parsing. Stores
/// the command line arguments and provides methods to find 
/// parameters with arguments (e.g. "-alpha 50") and check the 
/// existence of boolean parameters (like "-windowed").
#[derive(Debug)]
pub struct Options {
    args: Vec<String>
}

impl Options {
    pub fn new() -> Options {
        Options { args: env::args().collect() }
    }
    
    pub fn with_args(args: Vec<String>) -> Options {
        Options { args: args }
    }
    
    /// Return the parameter of the given argument. Returns None if the argument 
    /// is either not found, couldn't be parsed to the desired type or the next 
    /// argument is itself a commandline option (starts with '-').
    pub fn check_param<T>(&self, argument: &str) -> Option<T> where T: FromStr {  
        // Find the index of the argument in the argument list
        self.args.iter().position(|s| s == argument).and_then(|idx| {
            // Get the value occurring after that index 
            self.args.get(idx + 1).and_then(|arg| {
                // Don't return other commandline options
                if arg.starts_with("-") {
                    None
                } else {
                    // Try to parse the value to the desired type
                    arg.parse::<T>().ok()
                }
            })
        })
    }
    
    /// Checks if the given parameter is set.
    pub fn is_set(&self, param: &str) -> bool {
        for s in &self.args {
            if s == param {
                return true;
            }
        }
        
        false
    }
}

mod tests {
    #[test]
    fn test_parse_args() {
        let options = Options::with_args(vec!["-windowed".into(), "-alpha".into(), "50".into()]);
        let windowed = options.is_set("-windowed");
        assert_eq!(windowed, true);
        let alpha: Option<u32> = options.check_param("-alpha");
        assert_eq!(alpha, Some(50));
    }   
}