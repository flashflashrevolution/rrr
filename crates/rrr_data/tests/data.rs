#[cfg(test)]
mod data_tests {
    use rrr_data::songs::{Song, Songs};
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn validate_list_of_songs() {
        let song = r#"{
            "name": "Sea Breeze",
            "difficulty": 1,
            "time": 93,
            "id": 1485,
            "releasedate": 1310713418,
            "author": {
                "name": "championanwar",
                "url": "http://www.flashflashrevolution.com/profile/championanwar"
            },
            "stepauthor": {
                "name": "Silvuh",
                "url": "http://www.flashflashrevolution.com/profile/silvuh"
            },
            "rating": 4.2
        }"#;

        serde_json::from_str::<Song>(song).unwrap();

        let mut file = File::open("sample_data/sample_data.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        serde_json::from_str::<Songs>(&data).unwrap();
    }
}
