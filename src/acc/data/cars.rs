pub mod car_categories {
    use crate::model::CarCategory;

    pub const GT3: CarCategory = CarCategory::new("GT3");
    pub const GT4: CarCategory = CarCategory::new("GT4");
    pub const ST: CarCategory = CarCategory::new("ST");
    pub const ST22: CarCategory = CarCategory::new("ST");
    pub const CUP: CarCategory = CarCategory::new("CUP");
    pub const CUP21: CarCategory = CarCategory::new("CUP");
    pub const CHL: CarCategory = CarCategory::new("CHL");
    pub const TCX: CarCategory = CarCategory::new("TCX");
    pub const NONE: CarCategory = CarCategory::new("None");
}

pub mod cars {
    use crate::model::Car;

    use super::car_categories::*;

    pub const PORSCHE_991_GT3_R: Car = Car::new("Porsche 991 GT3 R", "Porsche", GT3);
    pub const MERCEDES_AMG_GT3_2015: Car = Car::new("Mercedes-AMG GT3", "Mercedes-AMG", GT3);
    pub const FERRARI_488_GT3: Car = Car::new("Ferrari 488 GT3", "Ferrari", GT3);
    pub const AUDI_R8_LMS: Car = Car::new("Audi R8 LMS", "Audi", GT3);
    pub const LAMBORGHINI_HURACAN_GT3: Car =
        Car::new("Lamborghini Huracan GT3", "Lamborghini", GT3);
    pub const MCLAREN_650S_GT3: Car = Car::new("McLaren 650S GT3", "McLaren", GT3);
    pub const NISSAN_GT_R_NISMO_GT3_2018: Car =
        Car::new("Nissan GT-R Nismo GT3 2018", "Nissan", GT3);
    pub const BMW_M6_GT3: Car = Car::new("BMW M6 GT3", "BMW", GT3);
    pub const BENTLEY_CONTINENTAL_GT3_2018: Car =
        Car::new("Bentley Continental GT3 2018", "Bentley", GT3);
    pub const PORSCHE_991_II_GT3_CUP: Car = Car::new("Porsche 991 II GT3 Cup", "Porsche", CUP);
    pub const NISSAN_GT_R_NISMO_GT3_2015: Car = Car::new("Nissan GT-R Nismo GT3", "Nissan", GT3);
    pub const BENTLEY_CONTINENTAL_GT3_2015: Car =
        Car::new("Bentley Continental GT3", "Bentley", GT3);
    pub const AMR_V12_VANTAGE_GT3: Car = Car::new("AMR V12 Vantage GT3", "Aston-Martin", GT3);
    pub const REITER_ENGINEERING_R_EX_GT3: Car =
        Car::new("Reiter Engineering R-EX GT3", "Reiter-Engineering", GT3);
    pub const EMIL_FREY_JAGUAR_G3: Car = Car::new("Emil Frey Jaguar G3", "Jaguar", GT3);
    pub const LEXUS_RC_F_GT3: Car = Car::new("Lexus RC F GT3", "Lexus", GT3);
    pub const LAMBORGHINI_HURACAN_GT3_EVO: Car =
        Car::new("Lamborghini Huracan GT3 Evo", "Lamborghini", GT3);
    pub const HONDA_NSX_GT3: Car = Car::new("Honda NSX GT3", "Honda", GT3);
    pub const LAMBORGHINI_HURACAN_ST: Car = Car::new("Lamborghini Huracan ST", "Lamborghini", ST);
    pub const AUDI_R8_LMS_EVO: Car = Car::new("Audi R8 LMS Evo", "Audi", GT3);
    pub const AMR_V8_VANTAGE: Car = Car::new("AMR V8 Vantage", "Aston-Martin", GT3);
    pub const HONDA_NSX_GT3_EVO: Car = Car::new("Honda NSX GT3 Evo", "Honda", GT3);
    pub const MCLAREN_720S_GT3: Car = Car::new("McLaren 720S GT3", "McLaren", GT3);
    pub const PORSCHE_911_II_GT3_R: Car = Car::new("Porsche 911 II GT3 R", "Porsche", GT3);
    pub const FERRARI_488_GT3_EVO: Car = Car::new("Ferrari 488 GT3 Evo", "Ferrari", GT3);
    pub const MERCEDES_AMG_GT3_2020: Car = Car::new("Mercedes-AMG GT3 2020", "Mercedes-AMG", GT3);
    pub const FERRARI_488_CHALLENGE_EVO: Car =
        Car::new("Ferrari 488 Challenge Evo", "Ferrari", CHL);
    pub const BMW_M2_CS_RACING: Car = Car::new("BMW M2 CS Racing", "BMW", TCX);
    pub const PORSCHE_: Car = Car::new("Porsche 992 GT3 CUP", "Porsche", CUP21);
    pub const LAMBORGHINI_HURACAN_ST_EVO2: Car =
        Car::new("Lamborghini Huracan ST EVO2", "Lamborghini", ST22);
    pub const BMW_M4_GT3: Car = Car::new("BMW M4 GT3", "BMW", GT3);
    pub const AUDI_R8_LMS_EVO2: Car = Car::new("Audi R8 LMS Evo 2", "Audi", GT3);
    pub const FERRARI_296_GT3: Car = Car::new("FERRARI 296 GT3", "Ferrari", GT3);
    pub const LAMBORGHINI_HURACAN_EVO2: Car =
        Car::new("Lamborghini Huracan EVO2", "Lamborghini", GT3);
    pub const PORSCHE_992_GT3_R: Car = Car::new("Porsche 992 GT3 R", "Porsche", GT3);
    pub const ALPINE_A110_GT4: Car = Car::new("Alpine A110 GT4", "Alpine", GT4);
    pub const ASTON_MARTIN_VANTAGE_GT4: Car =
        Car::new("Aston Martin Vantage GT4", "Aston-Martin", GT4);
    pub const AUDI_R8_LMS_GT4: Car = Car::new("Audi R8 LMS GT4", "Audi", GT4);
    pub const BMW_M4_GT4: Car = Car::new("BMW M4 GT4", "BMW", GT4);
    pub const CHEVROLET_CAMARO_GT4: Car = Car::new("Chevrolet Camaro GT4", "Chevrolet", GT4);
    pub const GINETTA_G55_GT4: Car = Car::new("Ginetta G55 GT4", "Ginetta", GT4);
    pub const KTM_X_BOW_GT4: Car = Car::new("KTM X-Bow GT4", "KTM", GT4);
    pub const MASERATI_MC_GT4: Car = Car::new("Maserati MC GT4", "Maserati", GT4);
    pub const MCLAREN_570S_GT4: Car = Car::new("McLaren 570S GT4", "McLaren", GT4);
    pub const MERCEDES_AMG_GT4: Car = Car::new("Mercedes AMG GT4", "Mercedes-AMG", GT4);
    pub const PORSCHE_718_CAYMAN_GT4_CLUBSPORT: Car =
        Car::new("Porsche 718 Cayman GT4 Clubsport", "Porsche", GT4);
    pub const ERROR: Car = Car::new("ERROR", "Error", NONE);
}
