#![allow(unused_imports)]
#![allow(dead_code)]

use std::{env, fs, fmt};
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::ffi::{CStr, OsStr};
use std::os::unix::ffi::OsStrExt;
use chrono::prelude::*;
use super::print_type;


pub trait Printable {
    fn print(&self) -> String;
}

pub struct FilePermission {
    pub name: String,
    pub mode: u32,
    pub uid: u32,
    pub username: String,
    pub gid: u32,
    pub groupname: String,
    pub size: u64,
    pub atime: String,
    pub mtime: String,
    pub ctime: String,
}

impl Printable for FilePermission{
    fn print(&self) -> String{
        let mut p = format!("NAME : {:?}\n", self.name);
        p += &format!("PERM : {:?}\n",format!("{:o}", self.mode & 0o000777));
        p += &format!("UID : {:?} - {}\n", self.uid, self.username);
        p += &format!("GID : {:?} - {}\n", self.gid, self.groupname);
        p += &format!("SIZE : {:?}\n", self.size);
        p += &format!("ATIME : {:?}\n", self.atime);
        p += &format!("MTIME : {:?}\n", self.mtime);
        p += &format!("CTIME : {:?}", self.ctime);

        p
    }
}


pub fn get_current_path() -> String{
    let path = env::current_dir().unwrap();

    path.into_os_string().into_string().unwrap()
}

pub fn get_file_list(path :&String) -> std::io::Result<Vec<String>>{
    let mut v: Vec<String> = Vec::new();

    for entry in fs::read_dir(path)? {
        let dir = entry?;
        v.push(dir.path().display().to_string());
    }
    Ok(v)
}

pub fn get_file_perm(f_name :String) -> std::io::Result<FilePermission>{
    let meta = std::fs::metadata(&f_name).unwrap();
    let perm = meta.permissions();

    let perms: FilePermission = FilePermission {
        name: f_name,
        mode: perm.mode() & 0o000777,
        uid: meta.uid(),
        username: get_username_by_uid(meta.uid()),
        gid: meta.gid(),
        groupname: get_groupname_by_gid(meta.gid()),
        size: meta.size(),
        atime: date_format(meta.atime()),
        mtime: date_format(meta.mtime()),
        ctime: date_format(meta.ctime())
    };

    Ok(perms)
}

fn date_format(timestamp :i64) -> String{
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    let datetime = Local.from_utc_datetime(&naive);
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    timestamp_str
}

fn get_username_by_uid(uid :u32) -> String {
    unsafe{
        let passwd :libc::passwd = libc::getpwuid(uid).read();
        let username = passwd.pw_name;
        let u_name = OsStr::from_bytes(CStr::from_ptr(username).to_bytes());

        u_name.to_str().unwrap().to_string()
    }
}

fn get_groupname_by_gid(gid :u32) -> String {
    unsafe{
        let group :libc::group = libc::getgrgid(gid).read();
        let groupname = group.gr_name;
        let g_name = OsStr::from_bytes(CStr::from_ptr(groupname).to_bytes());

        g_name.to_str().unwrap().to_string()
    }
}
