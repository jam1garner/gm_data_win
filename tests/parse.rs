const PATH: &str = "/home/jam/Documents/GameMakerStudio2/data.win";

//const PATH: &str = "data.win";

#[test]
fn try_parse() {
    gm_data_win::take_data_win_file(&std::fs::read(PATH).unwrap());
}
