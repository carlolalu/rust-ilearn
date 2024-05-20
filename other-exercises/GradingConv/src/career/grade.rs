use std::iter::successors;

// todo: "conversion tables for the grades between trieste and ljubljana"

lazy_static! {
    /// The universities/nationalities are ordered according to the chronology in which I encountered them.
    /// Notice that the marks for Italy go till 31: some universities count the 30 with honor key ('Lode') as a 31.
    static ref IT_GRADES : Vec<u32> = successors(Some(0), |&num| { Some(num+1) })
        .take(14)
        .map(|num| {(num+18) as u32} )
        .collect();

    static ref SLO_GRADES : Vec<f64> = successors(Some(0), |&num| { Some(num+1) })
        .take(10)
        .map(|num| (num as f64)/2_f64 + 6_f64 )
        .collect();

    static ref GER_GRADES : Vec<f64> = vec![4.0, 3.7, 3.3, 3.0, 2.7, 2.3, 2.0, 1.7, 1.3, 1.0];

    // must use vectors: hashmaps cannot take f64 as keys
    /// Such conversion is a standard of the unis and not of the countries.
    static ref TUE2TN : Vec<(f64, u32)> = {
        let v = vec![
        (4.0, 18),
        (3.7, 19),
        (3.3, 21),
        (3.0, 22),
        (2.7, 23),
        (2.3, 24),
        (2.0, 26),
        (1.7, 27),
        (1.3, 29),
        (1.0, 30),];
        v
    };

    // we use vectors here as well: symmetry with the previous data structure.
    /// Such conversion is a standard of teh unis and not of the countries.
    static ref TN2TUE : Vec<(u32, f64)> = {
        let v = vec![
        (18, 4.0),
        (19, 3.7),

        (20, 3.3),
        (21, 3.3),

        (22, 3.0),
        (23, 2.7),
        (24, 2.3),

        (25, 2.0),
        (26, 2.0),

        (27, 1.7),

        (28, 1.3),
        (29, 1.3),

        (30, 1.0),
        (31, 1.0),];
        v
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum Grade {
    /// The conversion table are always direct to the It system, thus that will be the pivot system.
    It(f64),
    Slo(f64),
    Ger(f64),
}

impl Grade {
    pub fn new_it(candidate_grade: u32) -> Result<Grade, &'static str> {
        let grade = Self::validate(
            candidate_grade as f64,
            IT_GRADES.iter().map(|num| *num as f64).collect::<Vec<_>>(),
        )?;
        Ok(Grade::It(grade as f64))
    }

    pub fn new_slo(candidate_grade: f64) -> Result<Grade, &'static str> {
        let grade = Self::validate(
            candidate_grade,
            SLO_GRADES.iter().map(|num| *num).collect::<Vec<_>>(),
        )?;
        Ok(Grade::Slo(grade))
    }

    pub fn new_ger(candidate_grade: f64) -> Result<Grade, &'static str> {
        let grade = Self::validate(candidate_grade as f64, GER_GRADES.to_vec())?;
        Ok(Grade::Ger(grade as f64))
    }

    fn validate(candidate_grade: f64, valid_grades: Vec<f64>) -> Result<f64, &'static str> {
        let output = match valid_grades.contains(&candidate_grade) {
            false => Err("Non-valid grade: is not among the allowed values"),
            true => Ok(candidate_grade as f64),
        };
        output
    }

    pub(super) fn get_it_grade(&self) -> f64 {
        match self {
            Grade::It(num) => *num,
            Grade::Slo(num) => *num * 3_f64,
            Grade::Ger(num) => {
                let index = GER_GRADES.iter().position(|item| *item == *num).unwrap();
                let tue_grade = TUE2TN[index].1;
                tue_grade as f64
            }
        }
    }

    pub(super) fn get_slo_grade(&self) -> f64 {
        self.get_it_grade() / 3_f64
    }

    pub(super) fn get_tue_grade(&self) -> f64 {
        let it_grade = self.get_it_grade() as u32;
        let index = IT_GRADES.iter().position(|item| *item == it_grade).unwrap();
        let tue_grade = TN2TUE[index].1;
        tue_grade
    }
}
