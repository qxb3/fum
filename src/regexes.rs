use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref FORWARD_RE: Regex = Regex::new(r"forward\((-?\d+)\)").unwrap();
    pub static ref BACKWARD_RE: Regex = Regex::new(r"backward\((-?\d+)\)").unwrap();

    pub static ref VOLUME_RE: Regex = Regex::new(r"volume\(([-+]?\d+)\)").unwrap();

    pub static ref VAR_RE: Regex = Regex::new(r"var\((\$\w[\w-]*),\s*(\$\w[\w-]*)\)").unwrap();
    pub static ref VAR_TOGGLE_RE: Regex = Regex::new(r"toggle\((\$\w[-\w]*),\s*(\$\w[-\w]*),\s*(\$\w[-\w]*)\)").unwrap();
    pub static ref VAR_SET_RE: Regex = Regex::new(r"set\((\$\w[-\w]*),\s*(\$\w[-\w]*)\)").unwrap();

    pub static ref GET_META_RE: Regex = Regex::new(r"get_meta\((.*?)\)").unwrap();

    pub static ref LOWER_RE: Regex = Regex::new(r"lower\(\s*([\w$-]+(?:\s+[\w$-]+)*)\s*\)").unwrap();
    pub static ref UPPER_RE: Regex = Regex::new(r"upper\(\s*([\w$-]+(?:\s+[\w$-]+)*)\s*\)").unwrap();
}
