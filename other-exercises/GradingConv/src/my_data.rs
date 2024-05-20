use crate::career::{exam::Exam, grade::Grade};


lazy_static! {
    /// The universities/nationalities are ordered according to the chronology in which I encountered them. First a list of all the 'heavy' exams.
    static ref TS_EXAMS : Vec<Exam> = {
        let v = vec![];
        v
    };

    /// I must retrieve here some grades here
    static ref LJ_EXAMS : Vec<Exam> = {
        let v = vec![
            Exam::new("Analiza 3", 6, Grade::new_slo(8_f64)).unwrap(),
            Exam::new("Optimizacija 1", 5, Grade::new_slo(9_f64)).unwrap(),
            // Exam::new("Analiza 2B", 4, Grade::new_slo()).unwrap(),
            Exam::new("Kodiranje in Kriptografija", 5, Grade::new_slo(8_f64)).unwrap(),
            Exam::new("Teorija Mere", 6, Grade::new_slo(7.5)).unwrap(),
            // Exam::new("Algebraicne Krivulje", 5, Grade::new_slo()).unwrap(),
            Exam::new("Analiza 4", 6, Grade::new_slo(8_f64)).unwrap(),
            ];
        v
    };

    static ref TN_EXAMS : Vec<Exam> = {
        let v = vec![
            Exam::new("Advanced Geometry", 9, Grade::new_it(31)).unwrap(),
            Exam::new("Stochastic Processes", 9, Grade::new_it(24)).unwrap(),

            Exam::new("Stochastic differential Equations", 6, Grade::new_it(28)).unwrap(),
            Exam::new("Type Theory", 6, Grade::new_it(27)).unwrap(),
            Exam::new("Numerical Methods for PDEs", 6, Grade::new_it(28)).unwrap(),
            Exam::new("Fourier Analysis", 6, Grade::new_it(26)).unwrap(),
            Exam::new("Functional Analysis", 6, Grade::new_it(27)).unwrap(),
            Exam::new("PDEs", 6, Grade::new_it(23)).unwrap(),
        ];
        v
    };

    static ref TUE_EXAMS : Vec<Exam> = {
        let v = vec![
            Exam::new("Geometry in Physics", 9, Grade::new_ger(1.3)).unwrap(),
            Exam::new("Mathematical Statistical Physics", 9, Grade::new_ger(1.3)).unwrap(),
            Exam::new("Seminar on derivation of effective equations", 3, Grade::new_ger(1.3)).unwrap(),
        ];
        v
    };

    /// A list of all exams which will not count for any final grade.
    static ref LIGHT_EXAMS : Vec<Exam> = {
        let v = vec![
            Exam::new("Foundations of Mathematics", 6, Grade::new_it(30)).unwrap(),

            // Exam::new("Algebraic Topology", 6, Grade::new_it()).unwrap(),
            Exam::new("Mathematische Statistik", 9, Grade::new_ger(2.7)).unwrap(),
            Exam::new("Seminar on Morse Homology", 3, Grade::new_ger(1.0)).unwrap(),
            Exam::new("Logik", 6, Grade::new_ger(2.7)).unwrap(),
        ];
        v
    };

    pub static ref HEAVY_EXAMS : Vec<Exam> = {
        let mut v = TUE_EXAMS.clone();
        v.append(&mut TN_EXAMS.clone());
        v
    };
}
