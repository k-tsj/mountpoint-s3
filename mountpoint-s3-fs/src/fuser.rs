//! Dummy implementations to replace mountpoint-s3-fuser dependency

use std::ffi::OsStr;
use std::time::{Duration, SystemTime};
use std::path::Path;
use std::os::fd::{RawFd, AsRawFd};
use std::fmt;

pub mod consts {
    pub const FOPEN_DIRECT_IO: u32 = 1;
    pub const FUSE_DO_READDIRPLUS: u32 = 1 << 13;
    pub const FUSE_ATOMIC_O_TRUNC: u32 = 1 << 3;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    NamedPipe,
    CharDevice,
    BlockDevice,
    Directory,
    RegularFile,
    Symlink,
    Socket,
}

#[derive(Debug, Clone, Copy)]
pub struct FileAttr {
    pub ino: u64,
    pub size: u64,
    pub blocks: u64,
    pub atime: SystemTime,
    pub mtime: SystemTime,
    pub ctime: SystemTime,
    pub crtime: SystemTime,
    pub kind: FileType,
    pub perm: u16,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub rdev: u32,
    pub blksize: u32,
    pub flags: u32,
}

impl Default for FileAttr {
    fn default() -> Self {
        Self {
            ino: 0,
            size: 0,
            blocks: 0,
            atime: SystemTime::UNIX_EPOCH,
            mtime: SystemTime::UNIX_EPOCH,
            ctime: SystemTime::UNIX_EPOCH,
            crtime: SystemTime::UNIX_EPOCH,
            kind: FileType::RegularFile,
            perm: 0o644,
            nlink: 1,
            uid: 0,
            gid: 0,
            rdev: 0,
            blksize: 4096,
            flags: 0,
        }
    }
}

#[derive(Debug)]
pub struct KernelConfig {
    pub max_write: u32,
    pub max_read: u32,
    pub capable: u32,
}

impl KernelConfig {
    pub fn add_capabilities(&mut self, cap: u32) -> Result<&mut Self, i32> {
        self.capable |= cap;
        Ok(self)
    }
    
    pub fn set_max_background(&mut self, _max_background: u16) -> Result<&mut Self, i32> {
        Ok(self)
    }
    
    pub fn set_congestion_threshold(&mut self, _congestion_threshold: u16) -> Result<&mut Self, i32> {
        Ok(self)
    }
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self {
            max_write: 131072,
            max_read: 131072,
            capable: 0,
        }
    }
}

impl fmt::Display for KernelConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "KernelConfig {{ max_write: {}, max_read: {}, capable: {} }}", 
               self.max_write, self.max_read, self.capable)
    }
}

// TimeOrNow enum
#[derive(Debug, Clone, Copy)]
pub enum TimeOrNow {
    SpecificTime(SystemTime),
    Now,
}

// Request type
pub struct Request<'a> {
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> Request<'a> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
    
    pub fn uid(&self) -> u32 {
        1000
    }
    
    pub fn gid(&self) -> u32 {
        1000
    }
    
    pub fn pid(&self) -> u32 {
        1
    }
    
    pub fn unique(&self) -> u64 {
        0
    }
    
    pub fn is_forget(&self) -> bool {
        false
    }
}

// Reply types
pub struct ReplyData;
impl ReplyData {
    pub fn data(&mut self, _data: &[u8]) {}
    pub fn error(&mut self, _error: i32) {}
}

pub struct ReplyEmpty;
impl ReplyEmpty {
    pub fn ok(&mut self) {}
    pub fn error(&mut self, _error: i32) {}
}

pub struct ReplyAttr;
impl ReplyAttr {
    pub fn attr(&mut self, _ttl: &Duration, _attr: &FileAttr) {}
    pub fn error(&mut self, _error: i32) {}
}

pub struct ReplyEntry;
impl ReplyEntry {
    pub fn entry(&mut self, _ttl: &Duration, _attr: &FileAttr, _generation: u64) {}
    pub fn error(&mut self, _error: i32) {}
}

pub struct ReplyOpen;
impl ReplyOpen {
    pub fn opened(&mut self, _fh: u64, _flags: u32) {}
    pub fn error(&mut self, _error: i32) {}
}

pub struct ReplyWrite;
impl ReplyWrite {
    pub fn written(&mut self, _size: u32) {}
    pub fn error(&mut self, _error: i32) {}
}

pub struct ReplyXTimes;
impl ReplyXTimes {
    pub fn xtimes(&mut self) {}
    pub fn error(&mut self, _error: i32) {}
}

pub struct ReplyStatfs;
impl ReplyStatfs {
    pub fn statfs(&mut self, _blocks: u64, _bfree: u64, _bavail: u64, _files: u64, 
                  _ffree: u64, _bsize: u32, _namelen: u32, _frsize: u32) {}
    pub fn error(&mut self, _error: i32) {}
}

pub struct ReplyReadlink;
impl ReplyReadlink {
    pub fn data(&mut self, _data: &[u8]) {}
    pub fn error(&mut self, _error: i32) {}
}

pub struct ReplyLseek;
impl ReplyLseek {
    pub fn offset(&mut self, _offset: i64) {}
    pub fn error(&mut self, _error: i32) {}
}

pub struct ReplyCreate;
impl ReplyCreate {
    pub fn created(&mut self, _ttl: &Duration, _attr: &FileAttr, _generation: u64, _fh: u64, _flags: u32) {}
    pub fn error(&mut self, _error: i32) {}
}

pub struct ReplyXattr;
impl ReplyXattr {
    pub fn size(&mut self, _size: u32) {}
    pub fn data(&mut self, _data: &[u8]) {}
    pub fn error(&mut self, _error: i32) {}
}

pub struct ReplyDirectory;
impl ReplyDirectory {
    pub fn add<T: AsRef<OsStr>>(&mut self, _ino: u64, _offset: i64, _kind: FileType, _name: T) -> bool {
        true
    }
    pub fn ok(&mut self) {}
    pub fn error(&mut self, _error: i32) {}
}

pub struct ReplyDirectoryPlus;
impl ReplyDirectoryPlus {
    pub fn add<T: AsRef<OsStr>>(&mut self, _ino: u64, _offset: i64, _name: T, 
                                _ttl: &Duration, _attr: &FileAttr, _generation: u64) -> bool {
        true
    }
    pub fn ok(&mut self) {}
    pub fn error(&mut self, _error: i32) {}
}

// Mount options
#[derive(Debug, Clone, PartialEq)]
pub enum MountOption {
    RO,
    AllowOther,
    AllowRoot,
    DefaultPermissions,
    FSName(String),
    NoAtime,
    AutoUnmount,
    Custom(String),
}

impl MountOption {
    pub fn option_str(&self) -> &str {
        match self {
            MountOption::RO => "ro",
            MountOption::AllowOther => "allow_other",
            MountOption::AllowRoot => "allow_root",
            MountOption::DefaultPermissions => "default_permissions",
            MountOption::FSName(_) => "fsname",
            MountOption::NoAtime => "noatime",
            MountOption::AutoUnmount => "auto_unmount",
            MountOption::Custom(s) => s,
        }
    }
}

// Session ACL
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SessionACL {
    All,
    RootAndOwner,
    Owner,
}

// Filesystem trait
pub trait Filesystem {
    fn init(&self, _req: &Request<'_>, _config: &mut KernelConfig) -> Result<(), i32> {
        Ok(())
    }
    
    fn destroy(&self) {}
    
    fn lookup(&self, _req: &Request<'_>, _parent: u64, _name: &OsStr, mut _reply: ReplyEntry) {}
    fn forget(&self, _req: &Request<'_>, _ino: u64, _nlookup: u64) {}
    fn getattr(&self, _req: &Request<'_>, _ino: u64, _fh: Option<u64>, mut _reply: ReplyAttr) {}
    fn setattr(&self, _req: &Request<'_>, _ino: u64, _mode: Option<u32>, _uid: Option<u32>, 
               _gid: Option<u32>, _size: Option<u64>, _atime: Option<TimeOrNow>, 
               _mtime: Option<TimeOrNow>, _ctime: Option<SystemTime>, _fh: Option<u64>, 
               _crtime: Option<SystemTime>, _chgtime: Option<SystemTime>, _bkuptime: Option<SystemTime>,
               _flags: Option<u32>, mut _reply: ReplyAttr) {}
    fn readlink(&self, _req: &Request<'_>, _ino: u64, mut _reply: ReplyData) {}
    fn mknod(&self, _req: &Request<'_>, _parent: u64, _name: &OsStr, _mode: u32, 
             _umask: u32, _rdev: u32, mut _reply: ReplyEntry) {}
    fn mkdir(&self, _req: &Request<'_>, _parent: u64, _name: &OsStr, _mode: u32, 
             _umask: u32, mut _reply: ReplyEntry) {}
    fn unlink(&self, _req: &Request<'_>, _parent: u64, _name: &OsStr, mut _reply: ReplyEmpty) {}
    fn rmdir(&self, _req: &Request<'_>, _parent: u64, _name: &OsStr, mut _reply: ReplyEmpty) {}
    fn symlink(&self, _req: &Request<'_>, _parent: u64, _name: &OsStr, _link: &Path, mut _reply: ReplyEntry) {}
    fn rename(&self, _req: &Request<'_>, _parent: u64, _name: &OsStr, _newparent: u64, 
              _newname: &OsStr, _flags: u32, mut _reply: ReplyEmpty) {}
    fn link(&self, _req: &Request<'_>, _ino: u64, _newparent: u64, _newname: &OsStr, mut _reply: ReplyEntry) {}
    fn open(&self, _req: &Request<'_>, _ino: u64, _flags: i32, mut _reply: ReplyOpen) {}
    fn read(&self, _req: &Request<'_>, _ino: u64, _fh: u64, _offset: i64, _size: u32, 
            _flags: i32, _lock_owner: Option<u64>, mut _reply: ReplyData) {}
    fn write(&self, _req: &Request<'_>, _ino: u64, _fh: u64, _offset: i64, _data: &[u8], 
             _write_flags: u32, _flags: i32, _lock_owner: Option<u64>, mut _reply: ReplyWrite) {}
    fn flush(&self, _req: &Request<'_>, _ino: u64, _fh: u64, _lock_owner: u64, mut _reply: ReplyEmpty) {}
    fn release(&self, _req: &Request<'_>, _ino: u64, _fh: u64, _flags: i32, 
               _lock_owner: Option<u64>, _flush: bool, mut _reply: ReplyEmpty) {}
    fn fsync(&self, _req: &Request<'_>, _ino: u64, _fh: u64, _datasync: bool, mut _reply: ReplyEmpty) {}
    fn opendir(&self, _req: &Request<'_>, _ino: u64, _flags: i32, mut _reply: ReplyOpen) {}
    fn readdir(&self, _req: &Request<'_>, _ino: u64, _fh: u64, _offset: i64, mut _reply: ReplyDirectory) {}
    fn releasedir(&self, _req: &Request<'_>, _ino: u64, _fh: u64, _flags: i32, mut _reply: ReplyEmpty) {}
    fn fsyncdir(&self, _req: &Request<'_>, _ino: u64, _fh: u64, _datasync: bool, mut _reply: ReplyEmpty) {}
    fn statfs(&self, _req: &Request<'_>, _ino: u64, mut _reply: ReplyStatfs) {}
    fn setxattr(&self, _req: &Request<'_>, _ino: u64, _name: &OsStr, _value: &[u8], 
                _flags: i32, _position: u32, mut _reply: ReplyEmpty) {}
    fn getxattr(&self, _req: &Request<'_>, _ino: u64, _name: &OsStr, _size: u32, mut _reply: ReplyXattr) {}
    fn listxattr(&self, _req: &Request<'_>, _ino: u64, _size: u32, mut _reply: ReplyXattr) {}
    fn removexattr(&self, _req: &Request<'_>, _ino: u64, _name: &OsStr, mut _reply: ReplyEmpty) {}
    fn access(&self, _req: &Request<'_>, _ino: u64, _mask: i32, mut _reply: ReplyEmpty) {}
    fn create(&self, _req: &Request<'_>, _parent: u64, _name: &OsStr, _mode: u32, 
              _umask: u32, _flags: i32, mut _reply: ReplyCreate) {}
    fn getlk(&self, _req: &Request<'_>, _ino: u64, _fh: u64, _owner: u64, _start: u64, 
             _end: u64, _typ: i32, _pid: u32, mut _reply: ReplyLock) {}
    fn setlk(&self, _req: &Request<'_>, _ino: u64, _fh: u64, _owner: u64, _start: u64, 
             _end: u64, _typ: i32, _pid: u32, _sleep: bool, mut _reply: ReplyEmpty) {}
    fn bmap(&self, _req: &Request<'_>, _ino: u64, _blocksize: u32, _idx: u64, mut _reply: ReplyBmap) {}
    fn ioctl(&self, _req: &Request<'_>, _ino: u64, _fh: u64, _flags: u32, _cmd: u32, 
             _in_data: &[u8], _out_size: u32, mut _reply: ReplyIoctl) {}
    fn fallocate(&self, _req: &Request<'_>, _ino: u64, _fh: u64, _offset: i64, 
                 _length: i64, _mode: i32, mut _reply: ReplyEmpty) {}
    fn lseek(&self, _req: &Request<'_>, _ino: u64, _fh: u64, _offset: i64, 
             _whence: i32, mut _reply: ReplyLseek) {}
    fn copy_file_range(&self, _req: &Request<'_>, _ino_in: u64, _fh_in: u64, _offset_in: i64,
                       _ino_out: u64, _fh_out: u64, _offset_out: i64, _len: u64, 
                       _flags: u32, mut _reply: ReplyWrite) {}
    #[cfg(target_os = "macos")]
    fn setvolname(&self, _req: &Request<'_>, _name: &OsStr, mut _reply: ReplyEmpty) {}
    #[cfg(target_os = "macos")]
    fn exchange(&self, _req: &Request<'_>, _parent1: u64, _name1: &OsStr, _parent2: u64, 
                _name2: &OsStr, _options: u64, mut _reply: ReplyEmpty) {}
    #[cfg(target_os = "macos")]
    fn getxtimes(&self, _req: &Request<'_>, _ino: u64, mut _reply: ReplyXTimes) {}
    fn readdirplus(&self, _req: &Request<'_>, _ino: u64, _fh: u64, _offset: i64, mut _reply: ReplyDirectoryPlus) {}
    #[cfg(feature = "abi-7-30")]
    fn rename2(&self, _req: &Request<'_>, _parent: u64, _name: &OsStr, _newparent: u64, 
               _newname: &OsStr, _flags: u32, mut _reply: ReplyEmpty) {}
    #[cfg(feature = "abi-7-30")]
    fn lseek(&self, _req: &Request<'_>, _ino: u64, _fh: u64, _offset: i64, 
             _whence: i32, mut _reply: ReplyLseek) {}
}

// Additional reply types that might be used
pub struct ReplyLock;
impl ReplyLock {
    pub fn locked(&mut self, _start: u64, _end: u64, _typ: i32, _pid: u32) {}
    pub fn error(&mut self, _error: i32) {}
}

pub struct ReplyBmap;
impl ReplyBmap {
    pub fn bmap(&mut self, _block: u64) {}
    pub fn error(&mut self, _error: i32) {}
}

pub struct ReplyIoctl;
impl ReplyIoctl {
    pub fn ioctl(&mut self, _result: i32, _data: &[u8]) {}
    pub fn error(&mut self, _error: i32) {}
}

// Session types
pub struct Session<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Filesystem> Session<T> {
    pub fn new(_fs: T, _mountpoint: &Path, _options: &[MountOption]) -> Result<Self, std::io::Error> {
        Ok(Self {
            _phantom: std::marker::PhantomData,
        })
    }
    
    pub fn from_fd<F: AsRawFd>(_fs: T, _fd: F, _acl: SessionACL) -> Result<Self, std::io::Error> {
        Ok(Self {
            _phantom: std::marker::PhantomData,
        })
    }
    
    pub fn run(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
    
    pub fn unmount_callable(&self) -> SessionUnmounter {
        SessionUnmounter
    }
    
    pub fn run_with_callbacks<B, A>(&self, _before: B, _after: A) -> Result<(), std::io::Error>
    where
        B: FnMut(&Request) + Send + 'static,
        A: FnMut(&Request) + Send + 'static,
    {
        // Dummy implementation - in real fuser this would process FUSE requests
        // and call the callbacks before and after each request
        Ok(())
    }
}

pub struct BackgroundSession;

impl BackgroundSession {
    pub fn new(_fs: impl Filesystem + Send + 'static + Sync, _mountpoint: &Path, _options: &[MountOption]) 
        -> Result<Self, std::io::Error> {
        Ok(Self)
    }
    
    pub fn join(self) {
        // Do nothing
    }
}

pub struct SessionUnmounter;

impl Drop for SessionUnmounter {
    fn drop(&mut self) {
        // Do nothing
    }
}

impl SessionUnmounter {
    pub fn unmount(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

pub struct Mount;

impl Mount {
    pub fn new(_mountpoint: &Path, _options: &[MountOption]) -> Result<Self, std::io::Error> {
        Ok(Self)
    }
}