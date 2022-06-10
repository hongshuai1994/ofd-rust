pub const INTERNAL_ERR: &str = "internal err.";
pub const FILENAME_EXTENSION_INVALID: &str = "filename extension invalid.";
pub const FILE_NOT_EXIST: &str = "file not exists.";
pub const FILE_OPEN_ERR: &str = "failed to open file.";
pub const INIT_ZIP_ERR: &str = "init zip file error.";
pub const UNZIP_ERR: &str = "err occurs during unzip";
pub const FILE_DOWNLOAD_ERR: &str = "failed to open file.";
pub const FILE_RENAME_ERR: &str = "failed to rename file.";
pub const FILE_COPY_ERR: &str = "failed to copy file.";
pub const CREATE_DIR_ERR: &str = "failed to create directory.";
pub const CREATE_FILE_ERR: &str = "failed to create file";

/// 处理 OFD 中遇到的错误
#[derive(Debug)]
pub struct Err {
    message: String,
    extra: String,
}

// Err 方法
impl Err {
    /// 打印 Err
    pub fn to_string(self) -> String {
        format!("\n===> err={} extra={} <===", self.message, self.extra)
    }

    /// 根据错误信息构造 Err
    pub fn from(message: &str, extra: String) -> Err {
        Err {
            message: String::from(message),
            extra,
        }
    }
}
