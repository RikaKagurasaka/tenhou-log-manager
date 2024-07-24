use quick_xml::events::BytesStart;

pub trait IntoNumVec<T> {
    fn into_num_vec(self) -> Vec<T>;
}

impl IntoNumVec<i32> for String {
    fn into_num_vec(self) -> Vec<i32> {
        self.split(',').map(|s| s.parse().unwrap()).collect()
    }
}

impl IntoNumVec<u8> for String {
    fn into_num_vec(self) -> Vec<u8> {
        self.split(',').map(|s| s.parse().unwrap()).collect()
    }
}

impl IntoNumVec<f32> for String {
    fn into_num_vec(self) -> Vec<f32> {
        self.split(',').map(|s| s.parse().unwrap()).collect()
    }
}

pub trait IntoActor {
    fn into_actor(self) -> u8;
}

impl IntoActor for char {
    fn into_actor(self) -> u8 {
        match self {
            'D' | 'T' => 0,
            'E' | 'U' => 1,
            'F' | 'V' => 2,
            'G' | 'W' => 3,
            _ => panic!("Invalid actor"),
        }
    }
}

pub trait GetAttribute {
    fn get_attribute(&self, key: &str) -> Option<String>;
}

impl<'a> GetAttribute for BytesStart<'a> {
    fn get_attribute(&self, key: &str) -> Option<String> {
        self.attributes()
            .find(|a| a.as_ref().unwrap().key.as_ref() == key.as_bytes())
            .map(|a| a.unwrap().value)
            .map(|v| String::from_utf8(v.to_vec()).unwrap())
    }
}
