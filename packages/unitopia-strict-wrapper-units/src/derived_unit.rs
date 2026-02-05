use typenum::{N1, P1};

pub type MulUnit<LhsUnit, RhsUnit> = (LhsUnit, P1, RhsUnit, P1);
pub type DivUnit<LhsUnit, RhsUnit> = (LhsUnit, P1, RhsUnit, N1);
