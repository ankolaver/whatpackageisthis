use crate::dates::calc_pkg_dates;
use colored::*;
use itertools::izip;
use scraper::{Html, Selector};

pub async fn scrape_web(url: String) {
    let url = url + "/issues";

    //connect to website
    let res = reqwest::get(url).await.unwrap().text().await.unwrap();

    //change to string type
    let str_res = res.as_str();

    //Use imbuilt method to convert object to readable key-value pair
    let frag_res = Html::parse_fragment(&str_res);

    //extract objects with class types corresponding to issues
    let selector = Selector::parse(r#"a[data-hovercard-type="issue"]"#).unwrap();

    println!(
        "\n{}",
        "Possible bugs from Github".truecolor(219, 112, 147).bold()
    );
    for element in frag_res.select(&selector) {
        let dat = element.text().collect::<Vec<_>>();
        println!("👆 {:?}", dat[0]);
    }
}

pub async fn scrape_version(url: String, pkg_version: String, build_date: String) {
    let url = url + "/tags";

    //connect to website
    let res = reqwest::get(url).await.unwrap().text().await.unwrap();

    //change to string type
    let str_res = res.as_str();

    //Use imbuilt method to convert object to readable key-value pair
    let frag_res = Html::parse_fragment(&str_res);

    //extract objects with class types corresponding to issues
    let version_selector =
        Selector::parse(r#"pre[class="text-small color-text-secondary"]"#).unwrap();

    //extract objects with class types corresponding to issues
    let date_selector = Selector::parse(r#"relative-time[class="no-wrap"]"#).unwrap();

    for (element1, element2) in izip!(
        frag_res.select(&version_selector),
        frag_res.select(&date_selector)
    ) {
        let version = element1.text().collect::<Vec<_>>();

        //Check against versions found in github
        if version[0].trim().contains(&pkg_version) {
            let release_date = element2.text().collect::<Vec<_>>();
            println!(
                " ❇️ Current Release Date {} ==> {} weeks behind the original Github release",
                &build_date[..],
                calc_pkg_dates(&build_date[..], release_date[0])
                    .num_weeks()
                    .to_string()
                    .truecolor(161, 121, 90)
            );
        }
    }
}
