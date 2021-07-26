pub mod file_structs;
pub use file_structs::take_data_win_file;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //let bytes = include_bytes!("/home/jam/a/re/rivals/data.win");
        dbg!(&bytes[..8]);
        dbg!(file_structs::take_data_win_file(bytes));
    }
}
