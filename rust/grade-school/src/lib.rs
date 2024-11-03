use std::collections::{BTreeMap, BTreeSet, HashSet};

pub struct School {
    grades: BTreeMap<u32, BTreeSet<String>>,
    students: HashSet<String>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: BTreeMap::new(),
            students: HashSet::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if !self.students.contains(student) {
            self.grades.entry(grade).or_default().insert(student.into());
            self.students.insert(student.into());
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().copied().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grades
            .get(&grade)
            .map(|grade| grade.iter().cloned().collect())
            .unwrap_or_else(Vec::new)
    }
}
