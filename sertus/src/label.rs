use once_cell::sync::Lazy;
use regex::Regex;

use crate::error::Result;

const LABEL_RE: Lazy<std::result::Result<Regex, regex::Error>> =
    Lazy::new(|| Regex::new(r"\((\w+)\s*,\s*(\w+)\)"));

pub trait LabelExtractor {
    fn extract_label(&self) -> Result<Vec<(String, String)>>;
}

impl LabelExtractor for String {
    fn extract_label(&self) -> Result<Vec<(String, String)>> {
        let mut labels = vec![];
        let label_re = LABEL_RE;
        let re = label_re.as_ref().map_err(|e| e.to_owned())?;
        self.lines().for_each(|line| {
            if line.starts_with("#label") {
                let line_labels = re
                    .captures_iter(line)
                    .map(|capture| (capture[1].to_string(), capture[2].to_string()))
                    .collect::<Vec<(String, String)>>();
                labels.extend(line_labels);
            }
        });
        Ok(labels)
    }
}
