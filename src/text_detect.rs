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
    // TODO: Test
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.instances.len() == 1 {
            return write!(f, "{}", self.instances.keys().last().unwrap());
        }

        let total: u32 = self.instances.values().sum();
        let mut result: Vec<String> = Vec::new();

        for (key, value) in self.instances.iter() {
            let percentage: f32 = (*value as f32/total as f32) * 100f32;
            let description = std::format!("{}: {}%", key, percentage);
            result.push(description);
        }
        write!(f, "{}", result.join(&"\n"))
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

