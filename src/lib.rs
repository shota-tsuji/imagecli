//! 画像のリサイズと統計情報表示に関するライブラリ
mod imagix;
use imagix::resize;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
