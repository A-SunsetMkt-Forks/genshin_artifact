use mona::character::CharacterName;
use crate::utils::character::{get_character_data, get_character_data_by_name};
use crate::utils::text_map::get_text_map;


fn get_traveller_translation(name: CharacterName, lang: &str) -> String {
    if name == CharacterName::AetherAnemo {
        match lang {
            "chs" => String::from("空（风）"),
            "eng" => String::from("Aether(Anemo)"),
            _ => unreachable!()
        }
    } else {
        unreachable!()
    }
}

pub fn gen_character_locale(lang: &str) -> String {
    let mut result = String::new();

    let text_map = get_text_map(lang);

    for i in 0_usize..CharacterName::LEN {
        let name: CharacterName = num::FromPrimitive::from_usize(i).unwrap();
        if name == CharacterName::AetherAnemo {
            result.push_str(&format!("AetherAnemo: \"{}\",\n", get_traveller_translation(name, lang)));
        } else {
            let data = get_character_data_by_name(name);
            result.push_str(&format!("{}: \"{}\",\n", name.to_string(), text_map.get(data.nameTextMapHash.to_string().as_str()).unwrap()));
        }

        // println!("{:?}", data);
    }

    result = String::from("export default {\n") + &result + "}";

    result
}