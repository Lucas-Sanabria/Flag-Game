use std::{ops::Deref, process::Command,error::Error,thread,time};
 use std::io::{stdin,stdout,Write};
use ::array_init::array_init;
 use viuer::{Config, print_from_file};
use rand;
struct FlagItem
    {
        title:String,
        url: String,
        filename: String,
    }
impl Default for FlagItem {
    fn default() -> Self { 
        Self {
        title:"".to_owned(),
        url:"".to_owned(), 
        filename: "".to_owned(),
        }
    }
}
fn termclear()
{
if cfg!(target_os = "windows") {
let cmd= std::process::Command::new("cls").status().unwrap();
}
    else {
            let he =Command::new("clear").status().unwrap();    
        }
}
fn main() 
    {
    let mut woo: [FlagItem;254]= array_init(|_| FlagItem::default());
    let ten_millis = time::Duration::from_millis(2000);
     let mut rng = rand::thread_rng();
     let indexes=rand::seq::index::sample(&mut rng, 254, 254);
     get_flags(&mut woo);
    let mut score=0;
    let mut i =0;
     let conf = Config {
    width: Some(45),
    height: Some(15),
    x: 10,
    y: 4,
    ..Default::default()
};

    let x= String::from("next");

      while i<254
 	{
     	let mut correct=0;
     	let  current=indexes.index(i);
        termclear();
    	print_from_file(&woo[current].filename, &conf).expect("Image printing failed.");
     	while  correct==0
        	{   let mut s= String::new();
            	let _=stdout().flush();
            	stdin().read_line(&mut s).expect("Did not enter a correct string");
                println!("{}",s); 
                let check = woo[current].title.eq_ignore_ascii_case(s.trim());
                let check2 = x.eq_ignore_ascii_case(s.trim());

                println!("{}", check);
             if check
            	{
             	correct=1;
             	println!("correct");
             	score=score+1;
         	}
             if check2 
             {
                 correct=1;
                 println!("{}",woo[current].title);
                 thread::sleep(ten_millis);
             } 
         	else {
             	println!("wrong");
         	}
    	i=i+1;
        	}

 	i=i+1;
 	}
print!("your score is {}",score);
 }

 fn get_flags(flag_list: &mut[FlagItem;254])
{   
    let response = reqwest::blocking::get(
        "https://flagpedia.net/index",
    )
    .unwrap()
    .text()
    .unwrap();
    let document = scraper::Html::parse_document(&response);
    let title_selector = scraper::Selector::parse(".flag-grid>li>a>span").unwrap();
    let flags = document.select(&title_selector).map(|x| x.inner_html());
    let link_selector = scraper::Selector::parse(".flag-grid>li>a>img").unwrap();
    let links = document.select(&link_selector).map(|x| x.value().attr("src"));
    flags
        .zip(0..254)
        .for_each(|(item, number)|{
        flag_list[number].title=item.to_owned().replace("-", " ").replace("Å", "A").replace("ô", "o").replace("í", "i").replace("é", "e").replace("ã","a").replace("'", "");
        flag_list[number].filename=(item.to_owned()+".png").replace("-", " ").replace(" ", "_").replace("Å", "A").replace("ô", "o").replace("í", "i").replace("é", "e").replace("ã","a");
        });

    links 
        .zip(0..254)
        .for_each(|(item, number )|{
            (flag_list[number].url="https://flagpedia.net".to_string()+item.unwrap());
         });
   // for x in flag_list{
     //  let testcomm = Command::new("wget").arg(x.url.as_str().replace("80", "160")).arg("-O")
        //.arg(x.filename.to_owned()).output();
         //println!("{}", x.title);
   // }
//uncomment these lines when you first run the program to scrape all the images needed to play
 }
 
