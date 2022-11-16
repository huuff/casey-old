use crate::case::Case;
use std::collections::HashMap;
use std::fmt;

pub struct DetectReport {
    instances: HashMap<Case, u32>
}

impl DetectReport {
    pub fn new(instances: HashMap<Case, u32>) -> Self {
       Self { instances } 
    }
}

impl fmt::Display for DetectReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let total: u32 = self.instances.values().sum();
        let mut result = String::new();

        // TODO: Display only one if there's one
        for (key, value) in self.instances.iter() {
            let percentage: f32 = (*value as f32/total as f32) * 100f32;
            // TODO: Add to a string and then join
            result.push_str(&std::format!("{}: {}%\n", key, percentage))
        }
        write!(f, "{}", result)
    } 
}

pub fn text_detect(text: &str) -> DetectReport {
    let mut result = HashMap::new();

    for line in text.lines() {
        for token in line.split_ascii_whitespace() {
            if let Some(case) = Case::detect(token) {
                if let Some(instances) = result.get(&case) {
                    result.insert(case, instances+1);
                } else {
                    result.insert(case, 1);
                }
            }
        }
    }

    DetectReport {
        instances: result,
    }
}

