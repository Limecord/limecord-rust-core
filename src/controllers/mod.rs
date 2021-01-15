#[macro_use]
pub mod tracking;

#[macro_export]
macro_rules! api_version_check {
    ($version:expr, $min_version:expr) => {
        let re = regex::Regex::new(r"^v(\d+)$").expect("regex");
        match re.find(&$version) {
            Some(found) => {
                println!("{:?}", found.text.parse::<i32>().unwrap());
            },
            None => {

            }
        }   
    }
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        tracking::science
    ]
}