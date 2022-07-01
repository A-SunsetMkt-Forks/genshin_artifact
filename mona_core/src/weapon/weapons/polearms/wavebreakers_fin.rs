use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct WavebreakersFinEffect {
    energy: usize
}

impl WavebreakersFinEffect {
    pub fn new(config: &WeaponConfig) -> WavebreakersFinEffect {
        match *config {
            WeaponConfig::WavebreakersFin { energy } => WavebreakersFinEffect {
                energy
            },
            _ => WavebreakersFinEffect {
                energy: 0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for WavebreakersFinEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let value = ((refine * 0.0003 + 0.0009) * self.energy as f64).min(refine * 0.1 + 0.3);
        attribute.set_value_by(AttributeName::BonusElementalBurst, "断浪长鳍被动", value);
    }
}

pub struct WavebreakersFin;

impl WeaponTrait for WavebreakersFin {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::WavebreakersFin,
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK30),
        weapon_base: WeaponBaseATKFamily::ATK620,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("队伍中所有角色的元素能量上限的总和，每1点能使装备此武器的角色的元素爆发造成的伤害提高0.12/0.15/0.18/0.21/0.24%，通过这种方式，元素爆发造成的伤害至多提高40/50/60/70/80%。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "断浪长鳍"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "energy",
            title: "w10",
            config: ItemConfigType::Int { min: 40, max: 400, default: 40 }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(WavebreakersFinEffect::new(config)))
    }
}
