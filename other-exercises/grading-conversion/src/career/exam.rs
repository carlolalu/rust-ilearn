pub use super::grade::Grade;

pub type Ects = u32;

#[derive(Debug, PartialEq, Clone)]
pub struct Exam {
    pub title: String,
    pub ects: Ects,
    pub grade: Grade,
}

impl Exam {
    pub fn new(
        title: &str,
        ects: Ects,
        candidate_grade: Result<Grade, &'static str>,
    ) -> Result<Exam, &'static str> {
        let output = match candidate_grade {
            Ok(grade) => Ok(Exam {
                title: title.to_string(),
                ects,
                grade,
            }),
            Err(str) => Err(str),
        };
        output
    }

    pub fn get_it_grade(&self) -> f64 {
        self.grade.get_it_grade()
    }

    pub fn get_slo_grade(&self) -> f64 {
        self.grade.get_slo_grade()
    }

    pub fn get_tue_grade(&self) -> f64 {
        self.grade.get_tue_grade()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn grades() {
        let expected = Ok(Grade::It(30_f64));
        let output = Grade::new_it(30);
        assert_eq!(expected, output);

        assert_eq!(
            Err("Non-valid grade: is not among the allowed values"),
            Grade::new_it(0)
        );
        assert_eq!(
            Err("Non-valid grade: is not among the allowed values"),
            Grade::new_it(32)
        );
        assert_eq!(
            Err("Non-valid grade: is not among the allowed values"),
            Grade::new_slo(5.5)
        );
        assert_eq!(
            Err("Non-valid grade: is not among the allowed values"),
            Grade::new_slo(7.3)
        );
        assert_eq!(
            Err("Non-valid grade: is not among the allowed values"),
            Grade::new_slo(11_f64)
        );
        assert_eq!(
            Err("Non-valid grade: is not among the allowed values"),
            Grade::new_ger(2.2)
        );
        assert_eq!(
            Err("Non-valid grade: is not among the allowed values"),
            Grade::new_ger(0.9)
        );
        assert_eq!(
            Err("Non-valid grade: is not among the allowed values"),
            Grade::new_ger(4.1)
        );

        assert_eq!(Ok(Grade::It(31_f64)), Grade::new_it(31));

        assert_eq!(Ok(Grade::Slo(8_f64)), Grade::new_slo(8_f64));
        assert_eq!(Ok(Grade::Slo(7.5)), Grade::new_slo(7.5));

        assert_eq!(Ok(Grade::Ger(2.0_f64)), Grade::new_ger(2.0));
    }

    #[test]
    fn exams() {
        let expected = Exam {
            title: "Algebraic Topology".to_string(),
            ects: 6,
            grade: Grade::Ger(2.0),
        };
        let output = Exam::new("Algebraic Topology", 6, Grade::new_ger(2.0));
        assert_eq!(Ok(expected), output);

        assert_eq!(
            Err("Non-valid grade: is not among the allowed values"),
            Exam::new("Algebraic Topology", 6, Grade::new_ger(7.0))
        );

        assert_eq!(
            2.0,
            Exam::new("Algebraic Topology", 6, Grade::new_ger(2.0))
                .unwrap()
                .get_tue_grade()
        );
        assert_eq!(
            26_f64,
            Exam::new("Algebraic Topology", 6, Grade::new_ger(2.0))
                .unwrap()
                .get_it_grade()
        );

        assert_eq!(
            2.0,
            Exam::new("Algebraic Topology", 6, Grade::new_it(25))
                .unwrap()
                .get_tue_grade()
        );
        assert_eq!(
            2.0,
            Exam::new("Algebraic Topology", 6, Grade::new_it(26))
                .unwrap()
                .get_tue_grade()
        );
    }
}
