use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
pub use crate::ExtendedPerms;
use crate::{ AsciiString, AsciiStr, AsciiChar, constants::* };


impl ExtendedPerms for Permissions {
    fn sugo(&self) -> u32{
        self.mode() & !S_IFMT
    }

    fn to_ascii(&self) -> AsciiString {
        pretty_perms(self.mode())
    }

    fn to_oct_string(&self) -> String {
        let val = self.sugo();
        format!("{:#o}", val)
    }

    fn is_file(&self) -> bool {
        S_IFREG & self.mode() > 0
    }

    fn is_dir(&self) -> bool {
        S_IFDIR & self.mode() > 0
    }

     fn is_link(&self) -> bool {
        S_IFLNK & self.mode() > 0
    }

    fn is_sticky(&self) -> bool {
        S_ISVTX & self.mode() > 0
    }

     fn is_sgid(&self) -> bool {
        S_ISGID & self.mode() > 0
    }

    fn is_suid(&self) -> bool {
        S_ISUID & self.mode() > 0
    }
}

//const PERMS: &'static AsciiStr = "rwxrwxrwx";
const IVALS: [u32;9] = [S_IRUSR, S_IWUSR, S_IXUSR, S_IRGRP, S_IWGRP, S_IXGRP, S_IROTH, S_IWOTH, S_IXOTH];

/// Given st_mode, return an ascii representation of the file
/// permissions, as one might get on the command line via, say,
/// `ls -l`
pub fn pretty_perms(val: u32) -> AsciiString {
    // Definition of stick bit, setuid and setgid taken from:
    // https://www.thegeekdiary.com/what-is-suid-sgid-and-sticky-bit/
    let perms = AsciiStr::from_ascii("rwxrwxrwx").unwrap();

    let is_dir = val & S_IFDIR > 0;

    let val = val & !S_IFMT;
    // initialize the fold with an AsciiString with 10 elements. The
    // first element represents the sticky bit. We initialize the last 9
    // chars to their respective values as determined by the perms var.
    let mut out = (0..9).fold( AsciiString::from_ascii("----------").unwrap(),
                           |mut sum, x| { if (val & IVALS[x]) > 0 {
                                                sum[x+1] = perms[x];
                                        }
                                        sum });
    /*
      0    1  2  3  4  5  6  7  8  9
    sticky ur uw ux gr gw gx or ow ox

    now we have to evalute the sticky bit
    Settings:
    T refers to when the owner execute permissions are off.
    t refers to when the owner execute permissions are on.
    */

    if is_dir {
        out[0] = AsciiChar::d;
        // sticky bit only relevant for directories
        if (val & S_ISVTX) > 0 {
            out[9] = if out[9] == AsciiChar::Minus { AsciiChar::T } else { AsciiChar::t };
        }
    }

    /*
    The setgid permission displays as an “s” in the group’s execute field.
    If a lowercase letter “l” appears in the group’s execute field,
    it indicates that the setgid bit is on, and the execute bit for the group is off or denied.
     */
    if (val & S_ISGID) > 0 {
        out[6] = if out[6] == AsciiChar::Minus { AsciiChar::l } else { AsciiChar::s };
    }
    /*
    The setuid permission displayed as an “s” in the owner’s execute field.
    If a capital “S” appears in the owner’s execute field, it indicates that
    the setuid bit is on, and the execute bit “x” for the owner of the
    file is off or denied.
     */
    if (val & S_ISUID) > 0 {
        out[9] = if out[9] == AsciiChar::Minus { AsciiChar::S } else { AsciiChar::s };
    }
    out
}

/// Get the bits involved in determining the file permissions.
/// That would be sticky/sgid/suid and owner, group, other bits
pub fn file_perms(input: u32) -> u32 {
    !S_IFMT & input
}

/// Get the bits involved in determining file type
pub fn file_type(input: u32) -> u32 {
    S_IFMT & input
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::os::unix::fs::PermissionsExt;

    //
    // other tests
    //

    #[test]
    fn pretty_perms_passed_0007() {
        assert_eq!(pretty_perms(0o0007), AsciiString::from_ascii("-------rwx").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0006() {
        assert_eq!(pretty_perms(0o0006), AsciiString::from_ascii("-------rw-").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0005() {
        assert_eq!(pretty_perms(0o0005), AsciiString::from_ascii("-------r-x").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0004() {
        assert_eq!(pretty_perms(0o0004), AsciiString::from_ascii("-------r--").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0003() {
        assert_eq!(pretty_perms(0o0003), AsciiString::from_ascii("--------wx").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0002() {
        assert_eq!(pretty_perms(0o0002), AsciiString::from_ascii("--------w-").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0001() {
        assert_eq!(pretty_perms(0o0001), AsciiString::from_ascii("---------x").unwrap());
    }

    //
    // group tests
    //

    #[test]
    fn pretty_perms_passed_0070() {
        assert_eq!(pretty_perms(0o0070), AsciiString::from_ascii("----rwx---").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0060() {
        assert_eq!(pretty_perms(0o0060), AsciiString::from_ascii("----rw----").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0050() {
        assert_eq!(pretty_perms(0o0050), AsciiString::from_ascii("----r-x---").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0040() {
        assert_eq!(pretty_perms(0o0040), AsciiString::from_ascii("----r-----").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0030() {
        assert_eq!(pretty_perms(0o0030), AsciiString::from_ascii("-----wx---").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0020() {
        assert_eq!(pretty_perms(0o0020), AsciiString::from_ascii("-----w----").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0010() {
        assert_eq!(pretty_perms(0o0010), AsciiString::from_ascii("------x---").unwrap());
    }

    //
    // Owner Tests
    //

    #[test]
    fn pretty_perms_passed_0700() {
        assert_eq!(pretty_perms(0o0700), AsciiString::from_ascii("-rwx------").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0600() {
        assert_eq!(pretty_perms(0o0600), AsciiString::from_ascii("-rw-------").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0500() {
        assert_eq!(pretty_perms(0o0500), AsciiString::from_ascii("-r-x------").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0400() {
        assert_eq!(pretty_perms(0o0400), AsciiString::from_ascii("-r--------").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0300() {
        assert_eq!(pretty_perms(0o0300), AsciiString::from_ascii("--wx------").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0200() {
        assert_eq!(pretty_perms(0o0200), AsciiString::from_ascii("--w-------").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0100() {
        assert_eq!(pretty_perms(0o0100), AsciiString::from_ascii("---x------").unwrap());
    }

    // all no sticky bit
    #[test]
    fn pretty_perms_passed_0777() {
        assert_eq!(pretty_perms(0o0777), AsciiString::from_ascii("-rwxrwxrwx").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0776() {
        assert_eq!(pretty_perms(0o0776), AsciiString::from_ascii("-rwxrwxrw-").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0775() {
        assert_eq!(pretty_perms(0o0775), AsciiString::from_ascii("-rwxrwxr-x").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0774() {
        assert_eq!(pretty_perms(0o0774), AsciiString::from_ascii("-rwxrwxr--").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0772() {
        assert_eq!(pretty_perms(0o0772), AsciiString::from_ascii("-rwxrwx-w-").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0771() {
        assert_eq!(pretty_perms(0o0771), AsciiString::from_ascii("-rwxrwx--x").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0751() {
        assert_eq!(pretty_perms(0o0751), AsciiString::from_ascii("-rwxr-x--x").unwrap());
    }

    // sticky bit
    #[test]
    fn pretty_perms_passed_1777_sticky_bit_on() {
        assert_eq!(pretty_perms(0o1777), AsciiString::from_ascii("trwxrwxrwx").unwrap());
    }

    #[test]
    fn pretty_perms_passed_1177_sticky_bit_on_owner_exe() {
        assert_eq!(pretty_perms(0o1177), AsciiString::from_ascii("t--xrwxrwx").unwrap());
    }

    #[test]
    fn pretty_perms_passed_1477_sticky_bit_on_owner_read() {
        assert_eq!(pretty_perms(0o1477), AsciiString::from_ascii("Tr--rwxrwx").unwrap());
    }

    #[test]
    fn pretty_perms_passed_1277_sticky_bit_on_owner_write() {
        assert_eq!(pretty_perms(0o1277), AsciiString::from_ascii("T-w-rwxrwx").unwrap());
    }

    #[test]
    fn pretty_perms_passed_1677_sticky_bit_on_owner_readwrite() {
        assert_eq!(pretty_perms(0o1677), AsciiString::from_ascii("Trw-rwxrwx").unwrap());
    }

    #[test]
    fn pretty_perms_passed_1077_sticky_bit_on_owner_off() {
        assert_eq!(pretty_perms(0o1077), AsciiString::from_ascii("T---rwxrwx").unwrap());
    }


    #[test]
    fn pretty_perms_passed_2751_sgid_on_group_exe_on() {
        assert_eq!(pretty_perms(0o2751), AsciiString::from_ascii("-rwxr-s--x").unwrap());
    }

    #[test]
    fn pretty_perms_passed_2741_sgid_on_group_exe_off() {
        assert_eq!(pretty_perms(0o2741), AsciiString::from_ascii("-rwxr-l--x").unwrap());
    }

    #[test]
    fn pretty_perms_passed_3751_sgid_on_group_exe_on() {
        assert_eq!(pretty_perms(0o3751), AsciiString::from_ascii("trwxr-s--x").unwrap());
    }

    #[test]
    fn pretty_perms_passed_3741_sgid_on_group_exe_off() {
        assert_eq!(pretty_perms(0o3741), AsciiString::from_ascii("trwxr-l--x").unwrap());
    }

    #[test]
    fn test_permissions() {
        let fname = "test_permissions.txt";
        let  f = File::create(fname).unwrap();
        let metadata = f.metadata().unwrap();
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o777);
        let mode = permissions.mode();
        std::fs::remove_file(fname).expect("unable to remove temp file");
        assert_eq!(mode, 0o777);
    }

    #[test]
    fn test_permissions_ugo() {
        let fname = "test_permissions_ugo.txt";
        let  f = File::create(fname).unwrap();
        let metadata = f.metadata().unwrap();
        let  permissions = metadata.permissions();
        //permissions.set_mode(0o777);
        // so is it a u32 or an i32????
        let pmode = permissions.mode();
        let mode = pretty_perms(pmode);
        std::fs::remove_file(fname).expect("unable to remove temp file");
        assert_eq!(mode, AsciiString::from_ascii("-rw-r--r--").unwrap());
        assert_eq!(pmode & !S_IFMT, 0o644);
    }

    #[test]
    fn test_permissions_get_oct_string() {
        let fname = "test_permissions_get_oct_string.txt";
        let  f = File::create(fname).unwrap();
        let metadata = f.metadata().unwrap();
        let  permissions = metadata.permissions();
        //permissions.set_mode(0o777);
        // so is it a u32 or an i32????
        let pmode = permissions.mode();
        let mode = permissions.to_oct_string();
        std::fs::remove_file(fname).expect("unable to remove temp file");
        assert_eq!(mode, String::from("0o644"));
        assert_eq!(pmode & !S_IFMT, 0o644);
    }

    #[test]
    fn test_file_perm_call_with_100644() {
        let perms = 0o100644;
        assert_eq!(file_perms(perms), 0o0644);
    }

    #[test]
    fn test_file_perm_call_with_100000() {
        let perms = 0o100000;
        assert_eq!(file_perms(perms), 0o0000);
    }

    #[test]
    fn test_file_type_call_with_100644() {
        let perms = 0o100644;
        assert_eq!(file_type(perms), 0o100000);
    }

    #[test]
    fn test_file_type_call_with_000777() {
        let perms = 0o000777;
        assert_eq!(file_type(perms), 0o000000);
    }

}