use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct SearchParamOptions {
    pub q: Option<String>,
    pub boost: Option<bool>,
    pub limit: Option<i32>,
}

impl SearchParamOptions {
    pub fn validate(&mut self) -> &Self {
        // Ensure limit is within the range [1, 50]
        self.limit = Some(self.limit.unwrap_or(10).clamp(1, 50));
        self
    }
}
