#[allow(unused_imports)]
use TestQuantity::*;
use strum::Display;

#[derive(Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum TestQuantity {
    PersonHeight,
    PersonShoulderWidth,
    PersonHipWidth,
    BuildingHeight,
    BuildingWidth,
    BuildingDepth,
    BuildingCountOfStories,
    PowerOfHydrogen,
    GalactosidaseActivityUnit,
}

impl TestQuantity {}
