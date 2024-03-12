use super::*;

impl TencerData {
    pub fn get_json_data(&self, key: &str) -> QueryResult<Option<String>> {
        use schema::*;

        json_data::table.find(key).select(json_data::value).first(&self.0).optional()
    }
}
