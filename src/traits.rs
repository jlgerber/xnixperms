use ascii::{AsciiString};
use std::os::unix::fs::PermissionsExt;

pub trait ExtendedPerms: PermissionsExt {
    /// get sticky, user, group, other as a u32
    fn sugo(&self) -> u32;
    /// retrieve the 10 character ascii representation
    /// of the permissions.
    ///
    /// In General, the last 9 characters are self explanatory.
    /// They appear in clusters of 3, whose values may be:
    /// - `r` for read
    /// - `w` for write
    /// - `x` for execute
    /// - `-` indicating the permission is not set
    ///
    /// # Example
    ///
    /// `-rwxr---w-`  indicates that the owner has read, write, and execute
    /// permissions. The group has read permissions, and others have write
    /// permissions (a particularly bad configuration mind you, but good
    /// for illustratiion)
    fn as_ascii(&self) -> AsciiString;
    fn is_file(&self) -> bool;
    fn is_dir(&self) -> bool;
    fn is_link(&self) -> bool;
    fn is_sticky(&self) -> bool;
    /// Is the Set Group ID set? If so, the executable will run
    /// under its group as opposed to the executor's group.
    fn is_sgid(&self) -> bool;

    /// Is the Set owner User ID upon execution set? IF true,
    /// the command (if it is a command) will run sa the owner's
    /// id instead of the user.
    fn is_suid(&self) -> bool;
}