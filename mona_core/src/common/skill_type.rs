use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum SkillType {
    NormalAttack,
    ChargedAttack,
    PlungingAttackLow,
    PlungingAttackHigh,
    PlungingAttackGround,
    ElementalSkill,
    ElementalBurst,
}
