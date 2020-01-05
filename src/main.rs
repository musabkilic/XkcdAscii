extern crate image;
extern crate clap;
extern crate reqwest;
extern crate scraper;
extern crate rss;

use std::io;
use std::io::Read;
use scraper::Html;
use scraper::Selector;
use std::fs::File;
use std::str::from_utf8;
use std::path::Path;
use clap::{Arg, App};


const GET_RANDOM_COMIC_URL : &str = "https://c.xkcd.com/random/comic/";
const GET_COMIC_URL : &str = "https://xkcd.com/";


fn intensity_to_ascii(value: &u8) -> &str {
    // changes an intensity into an ascii character
    // this is a central step in creating the ascii art
    let ascii_chars  = [
        " ", ".", "^", ",", ":", "_", "=", "~", "+", "O", "o", "*",
        "#", "&", "%", "B", "@", "$"
    ];

    let n_chars = ascii_chars.len() as u8;
    let step = 255u8 / n_chars;
    for i in 1..(n_chars - 1) {
        let comp = &step * i;
        if value < &comp {
            let idx = (i - 1) as usize;
            return ascii_chars[idx]
        }
    }

    ascii_chars[ (n_chars - 1) as usize ]
}

fn get_random_comic() -> String{
    return get_comic(GET_RANDOM_COMIC_URL);
}

fn get_comic(comic_url:&str) -> String{
    let mut res = reqwest::get(comic_url).unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).expect("[ERROR] Couldn't get data from the website!");

    let fragment = Html::parse_fragment(&body);
    let selector0 = Selector::parse("#comic").unwrap();
    let selector1 = Selector::parse("img").unwrap();

    for element0 in fragment.select(&selector0) {
        for element1 in element0.select(&selector1){
            let href = element1.value().attr("src");
            match href{
                None => (),
                Some(x) => {
                    return x.to_string()
                },
            }
        }
    }
    panic!("[ERROR] Comic doesn't exist!");
}

fn get_comic_id(comic_id:&str) -> String{
    return get_comic(&(GET_COMIC_URL.to_string()+comic_id+"/"));
}

fn save_comic(comic_image:String, image_name:&str){
    let mut resp = reqwest::get(&("https:".to_string()+&comic_image)).expect("request failed");
    let mut out = File::create(image_name).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}

fn main() {
    let matches = App::new("asciify")
                        .args( &[
                        Arg::from_usage("[comic_id] -id, --comic_id [id] 'Comic ID'"),
                        ])
                        .get_matches();

    let image = match matches.values_of_lossy("comic_id") {
        Some(v) => get_comic_id(&v.join("")),
        None => get_random_comic()
    };
    println!("{}", image);
    let name_split = image.split(".").collect::<Vec<&str>>();
    let image_name = &("comic.".to_owned()+name_split[name_split.len()-1]);
    save_comic(image, image_name);

    let img = match image::open(&Path::new(image_name)) {
        Ok(p) => p,
        Err(_e) => panic!("Not a valid image path or could no open image"),
    };

    // resize image as an option if its very large...defualts to screen width
    let dims = vec![320u32, 160u32];

    let img = img.resize_exact(dims[0], dims[1], image::FilterType::Nearest);

    // convert to LUMA and change each greyscale pixel into a character
    let imgbuf = img.to_luma();
    let ascii_art = imgbuf.pixels()
                    .map( |p| intensity_to_ascii(&p.data[0]) )
                    .fold( String::new(), |s, p| s + p );

    // we have one long string, but we need to chunk it by line
    let subs = ascii_art.as_bytes()
        .chunks(imgbuf.width() as usize)
        .map(from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();
    for s in subs {
        println!("{}", s);
    }
}
