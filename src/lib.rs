use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

use quick_xml::Reader;
use zip::ZipArchive;

use common::errs::*;
use common::extensions::*;
use common::ofd_elements::*;

pub mod reader;
pub mod writer;
pub mod common;

const DEFAULT_ZIP_EXTRACTION_DIR: &str = "./unzipDestDir";

/// OFD 结构体
#[derive(Debug)]
pub struct OFD {
    file_name: String,
    file_path: String,
    unzip_root_path: String,
    page_count: i32,
}

/// OFD 的方法
impl OFD {
    pub fn get_ofd_name(&self) -> &str {
        return self.file_name.as_ref();
    }

    pub fn get_ofd_path(&self) -> &str {
        return self.file_path.as_ref();
    }

    pub fn get_unzip_root_path(&self) -> &str {
        return self.unzip_root_path.as_ref();
    }

    pub fn get_page_count(&self) -> Result<i32, Err> {
        if self.page_count > 0 {
            return Ok(self.page_count);
        }
        if self.get_unzip_root_path().is_empty() {
            return Err(Err::from(MUST_NOT_EMPTY, format!("count page, but unzip path is empty")));
        }
        let root_ofd_xml_path = Path::new(self.get_unzip_root_path()).join(ROOT_OFD_XML_FILE_NAME);
        let _reader = match Reader::from_file(root_ofd_xml_path) {
            Err(why) => {
                return Err(Err::from(READ_XML_ERR, format!("scene={}, err={}", "count page", why.to_string())));
            }
            Ok(r) => r,
        };
        // TODO(待继续)
        Ok(0)
    }

    /// 加载本地 .ofd 文件初始化 OFD 实例
    pub fn from_local_file(ofd_path: &str) -> Result<OFD, Err> {
        let ofd_path = Path::new(ofd_path);
        // 判断文件扩展名
        if ofd_path.extension().is_none() || !ofd_path.extension().unwrap().eq(OFD_FILENAME_EXTENSION) {
            return Err(Err::from(FILENAME_EXTENSION_INVALID, format!("{:?} not a .ofd path", ofd_path)));
        }
        // 不校验是否真的是 zip file，直接解压；zip file 有特定的格式，如果不是 zip file，这里会报错
        let unzip_dest_path = Path::new(DEFAULT_ZIP_EXTRACTION_DIR).join(ofd_path.file_name().unwrap());
        if let Some(err) = unzip_to_dest_dir(ofd_path.to_str().unwrap(), unzip_dest_path.to_str().unwrap()) {
            return Err(Err::from(UNZIP_ERR, format!("why={:?}", err)));
        }
        Ok(OFD {
            file_name: String::from(ofd_path.file_name().unwrap().to_str().unwrap()),
            file_path: String::from(ofd_path.to_str().unwrap()),
            unzip_root_path: String::from(unzip_dest_path.to_str().unwrap()),
            page_count: 0,
        })
    }
}

/// 将指定路径的 zip 文件解压到指定路径
#[must_use = " err may occur during unzipping. this err must be handled. "]
fn unzip_to_dest_dir(zip_file_path: &str, dest_dir: &str) -> Option<Err> {
    let ofd_file = match File::open(zip_file_path) {
        Ok(f) => f,
        Err(why) => {
            return Some(Err::from(FILE_OPEN_ERR, format!("path={} why={}", zip_file_path, why.to_string())));
        }
    };
    // 不改扩展名，直接解压 ofd 文件
    let mut zip_file = match ZipArchive::new(ofd_file) {
        Ok(f) => f,
        Err(why) => {
            return Some(Err::from(INIT_ZIP_ERR, format!("why={}", why.to_string())));
        }
    };
    let dest_path = Path::new(dest_dir);
    if let Err(why) = fs::create_dir_all(dest_path) {
        return Some(Err::from(CREATE_DIR_ERR, format!("dir={} why={}", dest_dir, why.to_string())));
    }
    for i in 0..zip_file.len() {
        let mut file = match zip_file.by_index(i) {
            Ok(f) => f,
            Err(why) => return Some(Err::from(UNZIP_ERR, format!("why={}", why.to_string())))
        };
        let out_path = match file.enclosed_name() {
            Some(p) => dest_path.clone().join(p).to_owned(),
            None => continue,
        };
        if file.name().ends_with('/') {
            if let Err(why) = fs::create_dir_all(out_path) {
                return Some(Err::from(CREATE_DIR_ERR, format!("dir={} why={}", dest_dir, why.to_string())));
            }
        } else {
            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    if let Err(why) = fs::create_dir_all(&p) {
                        return Some(Err::from(CREATE_DIR_ERR, format!("dir={} why={}", dest_dir, why.to_string())));
                    }
                }
            }
            let mut outfile = match File::create(&out_path) {
                Ok(f) => f,
                Err(why) =>
                    return Some(Err::from(CREATE_FILE_ERR, format!("path={:?},why={}", out_path, why)))
            };
            if let Err(why) = io::copy(&mut file, &mut outfile) {
                return Some(Err::from(FILE_COPY_ERR, format!("why={}", why.to_string())));
            }
        }
    }
    None
}