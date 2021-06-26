//! 画像のresizeに関するモジュール
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::imagix::error::ImageXError;

/// 指定されたディレクトリ内の画像ファイルのパス一覧を返す関数
///
/// # Arguments
/// * `src_folder` - 対象のディレクトリの絶対パス
/// # Return
/// * 画像ファイルのパス一覧あるいはImageXError
pub fn get_image_files(src_folder: PathBuf) -> Result<Vec<PathBuf>, ImageXError> {
    let entries = fs::read_dir(src_folder)
        .map_err(|_e| ImageXError::UserInputError("Invalid source folder".to_string()))?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>(); // mapを適用した後にエラーにならなかったものを通す

    let image_entries = entries?
        .into_iter()
        .filter(|r| {
            r.extension() == Some("JPG".as_ref())
                || r.extension() == Some("jpg".as_ref())
                || r.extension() == Some("PNG".as_ref())
                || r.extension() == Some("png".as_ref())
        })
        .collect();

    Ok(image_entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    // ディレクトリのパスを受け取るとそのディレクトリ内の画像ファイルのパスを含むVectorが返る
    fn test_get_image_files_1() {
        const TARGET_DIR: &'static str = "/tmp/test/images/";
        let directory = PathBuf::from(TARGET_DIR);
        let file_image = PathBuf::from(TARGET_DIR.to_string() + "image_someone_1.jpg");
        let file_text = PathBuf::from(TARGET_DIR.to_string() + "file.txt");

        let _ = fs::create_dir_all(directory.as_path());
        let _ = fs::remove_file(file_image.as_path());
        let _ = File::create(file_image.as_path());
        let _ = File::create(file_text.as_path());

        assert_eq!(vec![file_image], get_image_files(directory).unwrap());
    }

    #[test]
    // 空のディレクトリのパスを受け取ると空のVectorが返る
    fn test_get_image_files_2() {
        const TARGET_DIR: &'static str = "/tmp/test/empty/";
        let directory = PathBuf::from(TARGET_DIR);
        let _ = fs::create_dir_all(directory.as_path());

        let expected: Vec<PathBuf> = Vec::new();

        assert_eq!(expected, get_image_files(directory).unwrap());
    }
}
