use indexmap::IndexMap;
use log::*;

pub struct IniSerializer {

}

pub const _TEST_INI_FILE: &str =
";comment
[section1]
key1=value1
key2=value2
[section2]
key3=value3
key4=value4
notavalidline";

pub type IniMap = IndexMap<String, IndexMap<String, String>>;

impl IniSerializer {
    pub fn new() -> Self {
        Self { }
    }

    pub fn deserialize(&self, input: &[u8]) -> IniMap {
        let mut result = IndexMap::new();
        let content = String::from_utf8_lossy(input);
        let mut current_section = String::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
                continue; // Skip empty lines and comments
            }

            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len() - 1].to_string();
                result.insert(current_section.clone(), IndexMap::new());
            } else if let Some(pos) = line.find('=') {
                let key = line[..pos].trim().to_string();
                let value = line[pos + 1..].trim().to_string();
                if let Some(section) = result.get_mut(&current_section) {
                    section.insert(key, value);
                }
            } else {
                warn!("Invalid line in INI file: {}", line);
            }
        }

        result
    }

    pub fn serialize(&self, data: &IniMap) -> String {
        let mut output = String::new();

        for (section, pairs) in data {
            output.push_str(&format!("[{}]\n", section));
            for (key, value) in pairs {
                output.push_str(&format!("{}={}\n", key, value));
            }
        }

        output
    }
}