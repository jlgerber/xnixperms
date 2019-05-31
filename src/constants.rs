#![allow(dead_code)]

/// Enum that wraps permissions for the particular system
pub const S_IFMT  : u32 =0o0170000; /* type of file */
pub const S_IFIFO : u32 =0o0010000; /* named pipe (fifo) */
pub const S_IFCHR : u32 =0o0020000; /* character special */
pub const S_IFDIR : u32 =0o0040000; /* directory */
pub const S_IFBLK : u32 =0o0060000; /* block special */
pub const S_IFREG : u32 =0o0100000; /* regular */
pub const S_IFLNK : u32 =0o0120000; /* symbolic link */
pub const S_IFSOCK: u32 =0o0140000; /* socket */
pub const S_IFWHT : u32 =0o0160000; /* whiteout */
pub const S_ISUID : u32 =0o0004000; /* set user id on execution */
pub const S_ISGID : u32 =0o0002000; /* set group id on execution */
pub const S_ISVTX : u32 =0o0001000; /* save swapped text even after use */
pub const S_IRUSR : u32 =0o0000400; /* read permission, owner */
pub const S_IWUSR : u32 =0o0000200; /* write permission, owner */
pub const S_IXUSR : u32 =0o0000100; /* execute/search permission, owner */
pub const S_IRWXG : u32 =0o0000070;    //mask for group permissions
pub const S_IRGRP : u32 =0o0000040;     //group has read permission
pub const S_IWGRP : u32 =0o0000020;     //group has write permission
pub const S_IXGRP : u32 =0o0000010;     //group has execute permission
pub const S_IRWXO : u32 =0o0000007;     //mask for permissions for others (not in group)
pub const S_IROTH : u32 =0o0000004;    //others have read permission
pub const S_IWOTH : u32 =0o0000002;     //others have write permission
pub const S_IXOTH : u32 =0o0000001;     //others have execute permission

pub const USR_MODE : u32 =  S_IRUSR | S_IWUSR | S_IXUSR;
pub const GRP_MODE : u32 =  S_IRGRP | S_IWGRP | S_IXGRP;
pub const OTH_MODE : u32 =  S_IROTH | S_IWOTH | S_IXOTH;
