use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref IPV4_MAPPING: HashMap<[u8; 4], u8> = {
        let mut map = HashMap::new();
        map.insert([255, 255, 255, 255], 32);
        map.insert([255, 255, 255, 254], 31);
        map.insert([255, 255, 255, 252], 30);
        map.insert([255, 255, 255, 248], 29);
        map.insert([255, 255, 255, 240], 28);
        map.insert([255, 255, 255, 224], 27);
        map.insert([255, 255, 255, 192], 26);
        map.insert([255, 255, 255, 128], 25);
        map.insert([255, 255, 255, 0], 24);
        map.insert([255, 255, 254, 0], 23);
        map.insert([255, 255, 252, 0], 22);
        map.insert([255, 255, 248, 0], 21);
        map.insert([255, 255, 240, 0], 20);
        map.insert([255, 255, 224, 0], 19);
        map.insert([255, 255, 192, 0], 18);
        map.insert([255, 255, 128, 0], 17);
        map.insert([255, 255, 0, 0], 16);
        map.insert([255, 254, 0, 0], 15);
        map.insert([255, 252, 0, 0], 14);
        map.insert([255, 248, 0, 0], 13);
        map.insert([255, 240, 0, 0], 12);
        map.insert([255, 224, 0, 0], 11);
        map.insert([255, 192, 0, 0], 10);
        map.insert([255, 128, 0, 0], 9);
        map.insert([255, 0, 0, 0], 8);
        map
    };
}
