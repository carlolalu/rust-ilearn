pub(super) mod exam;
pub mod grade;

use exam::{Exam, Ects};
use grade::Grade;

pub struct Career {
    pub exams: Vec<Exam>,
}

// todo: in the future you could create a way to call directly self defined closures (Oreilly: callbacks) so that anybody can execute its own functions on such grades and get the way to calculate the average that he wants
impl Career {
    pub fn add_exam(
        &mut self,
        title: &str,
        ects: Ects,
        candidate_grade: Result<Grade, &'static str>,
    ) -> Result<(), &'static str> {
        let candidate = Exam::new(title, ects, candidate_grade);
        match candidate {
            Err(e) => Err(e),
            Ok(exam) => Ok(self.exams.push(exam)),
        }
    }

    pub fn total_heavy_ects(&self) -> u32 {
        self.exams.iter().map(|exam| exam.ects).sum()
    }

    /// Common average for italian systems (but not for Trento):
    /// For every exam convert its grade to the italian system and then calculate the avg.
    /// (In this calculation the vote '30 with honor key' counts as a 31)
    pub fn avg_31_trento(&self) -> f64 {
        let weighted_sum: f64 = self
            .exams
            .iter()
            .map(|exam| exam.get_it_grade() * (exam.ects as f64))
            .sum();
        let avg = weighted_sum / (self.total_heavy_ects() as f64);
        avg
    }

    /// Starting grade for the final graduation process in Trento:
    /// First normalise avg_31_trento to 110, meaning: (avg_31_trento:30 = avg_110:110).
    /// Finally, since I have done a double degree program, I receive 1 bonus point which I sum to avg_110.
    pub fn initial_grade_graduation_trento(&self) -> f64 {
        let avg_30 = self.avg_31_trento();

        let bonus_point = 1_f64;
        let avg_110 = avg_30 * (110_f64 / 30_f64);

        avg_110 + bonus_point
    }

    /// Avg grade for the master according to Trento:
    /// For every exam convert its grade to the italian system and then calculate the avg.
    /// In this calculation the vote '30 with honor key' counts as a 30.
    pub fn avg_30_trento(&self) -> f64 {
        let weighted_sum: f64 = self
            .exams
            .iter()
            .map(|exam| {
                let mut grade = exam.get_it_grade().clone();
                if grade == 31_f64 {
                    grade = 30_f64;
                }
                grade * (exam.ects as f64)
            })
            .sum();
        let avg = weighted_sum / (self.total_heavy_ects() as f64);
        avg
    }

    pub fn avg_tuebingen(&self) -> f64 {
        let weighted_sum: f64 = self
            .exams
            .iter()
            .map(|exam| exam.get_tue_grade() * (exam.ects as f64))
            .sum();
        let avg = weighted_sum / (self.total_heavy_ects() as f64);
        avg
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::my_data;

    #[test]
    fn actual_trento_avg() {
        let actual_career = Career {
            exams: my_data::HEAVY_EXAMS.to_vec(),
        };
        println!(
            "My actual ects are (NON-iter): {}",
            actual_career.total_heavy_ects()
        );
        println!(
            "My actual carrer has this trento avg: {}",
            actual_career.avg_30_trento()
        );
    }

    #[test]
    fn actual_trento_graduation_starting_grade() {
        let actual_career = Career {
            exams: my_data::HEAVY_EXAMS.to_vec(),
        };
        println!(
            "My actual carrer has this trento starting grade for the graduation: {}",
            actual_career.initial_grade_graduation_trento()
        );
    }

    #[test]
    fn actual_tuebingen_avg() {
        let actual_career = Career {
            exams: my_data::HEAVY_EXAMS.to_vec(),
        };
        println!(
            "My actual carrer has this tuebingen avg: {}",
            actual_career.avg_tuebingen()
        );
    }
}