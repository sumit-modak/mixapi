use rand::Rng;
use std::collections::HashMap;
use std::error::Error;
use std::iter::zip;

pub struct Data {
    words: HashMap<u16, (String, String)>,
    pub layout_left: [[u8; 5]; 3],
    pub layout_right: [[u8; 5]; 3],
}

impl Data {
    pub fn new(mut a: std::env::Args) -> Result<Data, Box<dyn std::error::Error>> {
        a.next();
        let target_layout: String = match a.next() {
            Some(layout_name) => layout_name,
            None => return Err(Box::<dyn Error>::from("Insufficient Arguments")),
        };
        let target_layout: String =
            match std::fs::read_to_string(format!("layouts/{target_layout}")) {
                Ok(layout) => layout,
                Err(_) => return Err(Box::<dyn Error>::from("Layout Not Found")),
            };
        let default_layout: String = match std::fs::read_to_string(format!("layouts/default.conf"))
        {
            Ok(layout) => layout,
            Err(_) => return Err(Box::<dyn Error>::from("Default Layout Not Set")),
        };

        // Note: This hashmap is never needed throughout the program so it is not included in data struct
        let mut hmap: HashMap<u8, u8> = HashMap::with_capacity(30);
        let mut layout_left: [[u8; 5]; 3] = [[32; 5]; 3];
        let mut layout_right: [[u8; 5]; 3] = [[32; 5]; 3];

        for (i, (dl, tl)) in zip(default_layout.lines(), target_layout.lines()).enumerate() {
            for (j, (ds, ts)) in zip(dl.split_whitespace(), tl.split_whitespace()).enumerate() {
                let ds = ds.to_lowercase();
                let ts = ts.to_lowercase();
                if j <= 4 {
                    layout_left[i][j] = ts.parse::<char>().unwrap() as u8;
                } else {
                    layout_right[i][j - 5] = ts.parse::<char>().unwrap() as u8;
                }
                hmap.insert(
                    ts.parse::<char>().unwrap() as u8,
                    ds.parse::<char>().unwrap() as u8,
                );
            }
        }

        if hmap.len() != 30 {
            return Err(Box::<dyn Error>::from("Layout Not Set Properly"));
        }

        let mut app = Data {
            words: HashMap::with_capacity(1000),
            layout_left,
            layout_right,
        };
        app.generate(&hmap)?;
        Ok(app)
    }

    fn generate(&mut self, hmap: &HashMap<u8, u8>) -> Result<(), Box<dyn std::error::Error>> {
        let wordlist = std::fs::read_to_string("utils/wordlists/1k.txt");
        if let Err(_) = wordlist {
            return Err(Box::<dyn Error>::from("Wordlist Not Found"));
        } else {
            let wordlist = wordlist.unwrap();
            let mut s = wordlist.lines();
            for i in 0..999 {
                let s1 = s.next().unwrap().to_owned();
                let s2 = self.map_to_target(hmap, &s1);
                self.words.insert(i, (s1, s2));
            }
            Ok(())
        }
    }

    #[inline]
    fn map_to_target(&self, hmap: &HashMap<u8, u8>, s: &str) -> String {
        let mut new_s = String::with_capacity(s.len());
        for c in s.chars() {
            let k = hmap.get(&(c as u8));
            new_s.push(k.unwrap().to_owned() as char);
        }
        new_s
    }

    #[inline]
    pub fn get_pair(&self) -> (&String, &String) {
        let mut rng = rand::thread_rng();
        let res = self.words.get(&(rng.gen::<u16>() % 999)).unwrap();
        (&res.0, &res.1)
    }
}
