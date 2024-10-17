use color_eyre::eyre::eyre;
use color_eyre::Result;

#[derive(Debug, PartialEq, Eq)]
struct NestedKey {
    parent: String,
    child: Option<Box<NestedKey>>,
}

impl NestedKey {
    fn new(name: String) -> Self {
        Self { parent: name, child: None }
    }

    fn append(&mut self, key: NestedKey) {
        if let Some(ref mut nested) = self.child {
            nested.append(key)
        } else {
            self.child = Some(Box::new(key));
        }
    }
}

fn parse(key_chain: &str) -> Result<NestedKey> {
    let keys = key_chain.split('.').collect::<Vec<&str>>();
    if keys.is_empty() {
        return Err(eyre!("Empty key chain"));
    }

    let mut keys_iter = keys.iter();
    let root = keys_iter.next().unwrap();

    let mut nested = NestedKey::new(root.to_string());
    keys_iter.for_each(|key| nested.append(NestedKey::new(key.to_string())));

    Ok(nested)
}

#[cfg(test)]
mod tests {
    use super::{parse, NestedKey};

    #[test]
    fn test_parse_key_chains() {
        let key_chain = "key1.key2.key3.key4";

        let mut expected = NestedKey::new("key1".to_string());
        expected.append(NestedKey::new("key2".to_string()));
        expected.append(NestedKey::new("key3".to_string()));
        expected.append(NestedKey::new("key4".to_string()));

        let actual = parse(key_chain).unwrap();
        similar_asserts::assert_eq!(actual, expected);
    }
}
