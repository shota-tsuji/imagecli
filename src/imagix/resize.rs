//! 画像のresizeに関するモジュール
use std::fs;
use std::io;
use std::path::PathBuf;

use super::error::ImageXError;

// @param 対象のディレクトリの絶対パス
// @return 画像ファイルのパス一覧あるいはImageXError
/// 指定されたディレクトリ内の画像ファイルのパス一覧を返す関数
pub fn get_image_files(src_folder: PathBuf) -> Result<Vec<PathBuf>, ImageXError> {
    let mut vec = Vec::new();
    let file = PathBuf::from("/tmp/images/image_someone_1.jpg");
    vec.push(file);

    let entries = fs::read_dir(src_folder)
        .map_err(|_e| ImageXError::UserInputError("Invalid source folder".to_string()))?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()? // mapを適用した後にエラーにならなかったものを通す
        .into_iter()
        .filter(|r| {
            r.extension() == Some("JPG".as_ref())
                || r.extension() == Some("jpg".as_ref())
                || r.extension() == Some("PNG".as_ref())
                || r.extension() == Some("png".as_ref())
        })
        .collect();
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // ディレクトリの絶対パスを受け取ってそのディレクトリに含まれる画像ファイルのパスをVectorとして返す
    fn test_get_image_files_1() {
        let directory = PathBuf::from("/tmp/images/");
        let file = PathBuf::from("/tmp/images/image_someone_1.jpg");
        let expected = vec![file];

        assert_eq!(expected, get_image_files(directory));
    }
}
