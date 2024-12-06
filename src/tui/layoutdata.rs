use rand::Rng;
use std::collections::HashMap;
use std::iter::zip;

pub struct LayoutData {
    // contains all the unmapped and mapped words according to the default and target layout
    words: HashMap<u16, (String, String)>,
    // mapped target layout left part
    pub layout_left: [[u8; 5]; 3],
    // mapped target layout right part
    pub layout_right: [[u8; 5]; 3],
}

impl LayoutData {
    pub fn new(a: String) -> Result<LayoutData, &'static str> {
        let target_layout = match std::fs::read_to_string(format!("utils/layouts/{a}")) {
            Ok(layout) => layout,
            Err(_) => return Err("Layout Not Found"),
        };
        let default_layout = match std::fs::read_to_string(format!("utils/layouts/default.conf")) {
            Ok(layout) => layout,
            Err(_) => return Err("Default Layout Not Set"),
        };

        // Note: This hashmap is never needed throughout the program so it is not included in data struct
        let mut hmap: HashMap<u8, u8> = HashMap::with_capacity(30);
        let mut layout_left: [[u8; 5]; 3] = [[32; 5]; 3];
        let mut layout_right: [[u8; 5]; 3] = [[32; 5]; 3];

        // generating mapped layouts according to the target layout
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
            return Err("Layout Not Set Properly");
        }

        // creating the mapped wordlist
        let mut words = HashMap::with_capacity(1000);
        if let Ok(wordlist) = std::fs::read_to_string("utils/wordlists/1k.txt") {
            let mut s = wordlist.lines();
            for i in 0..999 {
                let s1 = s.next().unwrap().to_owned();
                let mut s2 = String::with_capacity(s1.len());
                for c in s1.chars() {
                    let k = hmap.get(&(c as u8));
                    s2.push(k.unwrap().to_owned() as char);
                }
                words.insert(i, (s1, s2));
            }
        } else {
            return Err("Wordlist Not Found");
        }

        Ok(LayoutData {
            words,
            layout_left,
            layout_right,
        })
    }

    pub fn get_random_pair(&self) -> (&String, &String) {
        let mut rng = rand::thread_rng();
        let res = self.words.get(&(rng.gen::<u16>() % 999)).unwrap();
        (&res.0, &res.1)
    }
}
