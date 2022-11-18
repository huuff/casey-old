use crate::case::Case;
use std::collections::HashMap;

pub struct DetectReport {
    instances: HashMap<Case, u32>
}

impl DetectReport {
    pub fn new() -> Self {
       Self { instances: HashMap::new() } 
    }

    pub fn register(&mut self, case: Case) {
        if let Some(instances) = self.instances.get(&case) {
            self.instances.insert(case, instances+1);
        } else {
            self.instances.insert(case, 1);
        }
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

    pub fn long_description(&self) -> String {
        if self.instances.len() == 1 {
            return format!("{}", self.instances.keys().last().unwrap());
        } else if self.instances.len() == 0 {
            return String::from("Couldn't detect a case");
        }

        let total: u32 = self.instances.values().sum();
        let mut result: Vec<String> = Vec::new();

        let mut ordered_entries: Vec<(&Case, &u32)>
            = self.instances.iter()
                            .collect::<Vec<(&Case, &u32)>>()
                            ;
        ordered_entries.sort_by(|t1, t2| t2.1.cmp(t1.1));

        for (key, value) in ordered_entries {
            let percentage: f32 = (*value as f32/total as f32) * 100f32;
            let description = std::format!("{}: {}%", key, percentage);
            result.push(description);
        }
        format!("{}", result.join(&"\n"))
    } 
}

pub fn detect_text(text: &str, report: &mut DetectReport) {
    for token in text.split_ascii_whitespace() {
        if let Some(case) = Case::detect(token) {
            report.register(case);
        }
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
