
mod settings;
use crate::settings::Settings;
use rusindo::services::jwt::Jwt;



fn main()  {


    let settings = Settings::new().unwrap();
    println!("{:?}", settings);

    let jwt = Jwt::new(settings.jwt);
    let (token, r) = jwt.general_token_pair("sub".to_string());
    println!("{:?}", token);
    println!("{:?}", r);

    let claims = jwt.validate_access_token(&token);
    println!("{:?}", claims);


}