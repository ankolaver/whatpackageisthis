
use colored::*;

use scraper::{Html, Selector};


pub async fn scrape_web(url : String){
    let url = url+"/issues";

    //connect to website
    let res = reqwest::get(url).await.unwrap().text().await.unwrap();

    //change to string type
    let str_res = res.as_str();
    
    //Use imbuilt method to convert object to readable key-value pair
    let frag_res = Html::parse_fragment(&str_res);

    //extract objects with class types corresponding to issues
    let selector = Selector::parse(r#"a[data-hovercard-type="issue"]"#).unwrap();

    println!("\n{}","Possible bugs from Github".truecolor(219,112,147).bold());
    for element in frag_res.select(&selector){
        let dat = element.text().collect::<Vec<_>>();
        println!("👆 {:?}", dat[0]);
        //datas.push(dat);
    }
}