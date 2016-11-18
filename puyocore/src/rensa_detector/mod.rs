#[cfg(all(target_feature = "avx2", target_feature = "bmi2"))]
pub mod detector;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurposeForFindingRensa {
    ForFire,
    ForKey,
}
