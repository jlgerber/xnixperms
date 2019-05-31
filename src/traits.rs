use ascii::{AsciiString};
use std::os::unix::fs::PermissionsExt;

pub trait ExtendedPerms: PermissionsExt {
    fn sugo(&self) -> u32;
    fn as_ascii(&self) -> AsciiString;
    fn is_file(&self) -> bool;
    fn is_dir(&self) -> bool;
    fn is_link(&self) -> bool;
    fn is_sticky(&self) -> bool;
    fn is_sgid(&self) -> bool;
    fn is_suid(&self) -> bool;

}