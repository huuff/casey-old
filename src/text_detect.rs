use crate::case::Case;
use std::collections::HashMap;

pub struct DetectReport {
    instances: HashMap<Case, u32>
}

impl DetectReport {
    pub fn new(instances: HashMap<Case, u32>) -> Self {
       Self { instances } 
    }

    pub fn main_case(&self) -> Option<Case> {
        self.instances.iter()
            .max_by(|x, y| x.1.cmp(y.1))
            .map(|(key, _)| key)
            .map(|case| case.clone())
    }

    pub fn short_description(&self) -> String {
        if let Some(case) = self.main_case() {
            format!("{}", case.to_string())
        } else {
            String::from("Couldn't detect case")
        }
    }

    // TODO: Order by number of instances!
    pub fn long_description(&self) -> String {
        if self.instances.len() == 1 {
            return format!("{}", self.instances.keys().last().unwrap());
        }

        let total: u32 = self.instances.values().sum();
        let mut result: Vec<String> = Vec::new();

        for (key, value) in self.instances.iter() {
            let percentage: f32 = (*value as f32/total as f32) * 100f32;
            let description = std::format!("{}: {}%", key, percentage);
            result.push(description);
        }
        format!("{}", result.join(&"\n"))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_case() {
        // ARRANGE
        let mut instances = HashMap::new();
        instances.insert(Case::Camel, 3);
        instances.insert(Case::Snake, 2);
        instances.insert(Case::Pascal, 1);
        let report = DetectReport { instances };

        // ACT
        let result = report.main_case();

        // ASSERT
        assert_eq!(Case::Camel, result.unwrap());
    }

    #[test]
    fn short_description_not_found() {
        // ARRANGE
        let report = DetectReport { instances: HashMap::new() };

        // ACT
        let result = report.short_description();

        // ASSERT
        assert_eq!("Couldn't detect case", result);
    }

    #[test]
    fn short_description() {
        // ARRANGE
        let mut instances = HashMap::new();
        instances.insert(Case::Camel, 3);
        instances.insert(Case::Snake, 2);
        instances.insert(Case::Pascal, 1);
        let report = DetectReport { instances };

        // ACT
        let result = report.short_description();

        // ASSERT
        assert_eq!("camelCase", result);
    }

    #[test]
    fn long_description() {
        // ARRANGE
        let mut instances = HashMap::new();
        instances.insert(Case::Camel, 1);
        instances.insert(Case::Snake, 1);
        instances.insert(Case::Pascal, 1);
        let report = DetectReport { instances };

        // ACT
        let result = report.long_description();

        // ASSERT
        assert!(result.contains("camelCase: 33"));
        assert!(result.contains("snake_case: 33"));
        assert!(result.contains("PascalCase: 33"));
    }

}
