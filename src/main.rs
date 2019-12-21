use std::time::{SystemTime, UNIX_EPOCH};
use reqwest;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use scraper::{Selector, Html};

extern crate regex;
use regex::Regex;

use std::env;
use std::fs;

use reqwest::header;

fn main() -> Result<(), Box<std::error::Error>> {
    // let start = SystemTime::now();
    // let since_the_epoch = start.duration_since(UNIX_EPOCH)
    //     .expect("Time went backwards");


 
    let mut headers = header::HeaderMap::new();
    headers.insert("Cookie", header::HeaderValue::from_static("coc=1; uuid=aae3fd7f6ec688fb0fbdaa8b819fdc75; PHPSESSID=a1nkjn031sqsh0ueqlat7c7s00; _ga=GA1.2.971513544.1576930230; _gid=GA1.2.1376111597.1576930230; SL_GWPT_Show_Hide_tmp=1; SL_wptGlobTipTmp=1; __ulfpc=201912212010305390; adc=1"));
    
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let mut resp = client.get( "https://www.mgstage.com/search/search.php?sale_start_range=2019.12.20-2019.12.20").send();

    let body = resp.unwrap().text().unwrap();

    let document = Html::parse_document(&body);
    for product_a in document.select(&Selector::parse("h5 > a").unwrap()) {
        

         //"/product/product_detail/300MAAN-508/" //"/product/product_detail/300MAAN-508/"
        let product_href   = product_a.value().attr("href").unwrap();
        let product_img    = product_a.select(&Selector::parse("img").unwrap()).next().unwrap().value().attr("src").unwrap();
        let v: Vec<&str>   = product_href.split("/").collect();
        let product_no     = format!("{}_sample.mp4",v[3]);

        // println!("{}",product_href);   
        // println!("{}",product_img);

        //"https://sample.mgstage.com/sample/jackson/390jac/023/pf_t1_390jac-023.jpg";
        let sample_mp4 = product_img.replace("image.mgstage.com","sample.mgstage.com").replace("images","sample");
        // println!("{}",sample_mp4);
        
        let re      = Regex::new("^.*/").unwrap();
        let mut url_prefix = String::from("");
        for cap in re.captures_iter(&sample_mp4) {
            url_prefix = format!("{}",&cap[0]);
        }
        println!("{}{}",url_prefix,product_no);
        // https://sample.mgstage.com/sample/scute/229scute/965/scute_965.mp4
        // https://sample.mgstage.com/sample/scute/229scute/965/229SCUTE-965_sample.mp4
        // https://sample.mgstage.com/sample/shirouto/siro/4015/SIRO-4015_sample.mp4
        // https://sample.mgstage.com/sample/shirouto/siro/4015/SIRO-4015_sample.mp4
    }

//     let html_page_detail = fs::read_to_string("detail.html")
//         .expect("Something went wrong reading the file");
//     // println!("{}",html_page_detail);

    
//     let mut document     = Html::parse_document(&html_page_detail);
//     // let m_title_selector = Selector::parse().unwrap();
//     // let m_title          = document.select(&m_title_selector).next().unwrap();
    

//     let m_title          = get_inner_html("div.common_detail_cover > h1".to_string(),&document);
//     let m_login_times    = get_inner_html("div.detail_left > div > table:nth-child(2) > tbody > tr:nth-child(1) > td:nth-child(2)".to_string(),&document);
    
//     let m_author         = get_inner_html("div.common_detail_cover > div.detail_left > div > table:nth-child(3) > tbody > tr:nth-child(1) > td > a".to_string(),&document);
//     let m_company        = get_inner_html("div.common_detail_cover > div.detail_left > div > table:nth-child(3) > tbody > tr:nth-child(2) > td > a".to_string(),&document);

//     let m_duration       = get_inner_html("div.common_detail_cover > div.detail_left > div > table:nth-child(3) > tbody > tr:nth-child(3) > td".to_string(),&document);
//     // let m_product_no       =
//     // let m_sell_start     =
//     // let m_series         = 
//     // let m_level         = 
//     // let m_category      = 
//     // let m_star          = \
//     // let m_sample        = 
//     // let m_introduce      = 
//     // let m_images          = 

//     // let m_pingjias      = 
//     // println!("{}", m_title.inner_html() );
    
//     let m_pic_main      = "#center_column > div.common_detail_cover > div.detail_left > div > div > h2 > img";
//     let selector_a_imgs = Selector::parse("a.sample_image").unwrap();
//     for element in document.select(&selector_a_imgs) {

//         let img_small   = element.value().attr("href").unwrap();
//         let img_normal  = element.select(&Selector::parse("img").unwrap()).next().unwrap().value().attr("src").unwrap();
        
//         println!("{}",img_normal);
// // 200GANA-2215
// //         //https://www.mgstage.com/product/product_detail/390JAC-023/#user_review
// //         https://image.mgstage.com/images/jackson/390jac/023/cap_t1_20_390jac-023.jpg
// //         https://sample.mgstage.com/sample/jackson/390jac/023/390JAC-023_sample.mp4
// //         // https://sample.mgstage.com/sample/jackson/390jac/023/390JAC-023_sample.mp4
//     }


//     // https://image.mgstage.com/images/nanpatv/200gana/2215/cap_e_0_200gana-2215.jpg

//     let node1     = document.select(&Selector::parse("source").unwrap()).next().unwrap();
//     println!("{}",node1.inner_html());
    Ok(())
}

pub fn get_inner_html( selector_str:String , document: &scraper::html::Html ) ->  String {
    let selector = Selector::parse(&selector_str).unwrap();
    let node     = document.select(&selector).next().unwrap();
    let html     = node.inner_html(); //.trim()
    println!("{}",html);
    return html;
}