use chrono::{Duration, Utc};
use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    fn new(aud: String, sub: String, exp: usize, iat: usize) -> Self {
        Claims { aud, sub, exp, iat }
    }
}

enum TokenKind {
    ACCESS,
    REFRESH,
}

impl Jwt {
    pub fn general_token_pair(&self, sub: String) -> (String, String) {
        let access_token = self.general_token(TokenKind::ACCESS, sub.clone());
        let refresh_token = self.general_token(TokenKind::REFRESH, sub);
        (access_token, refresh_token)
    }

    pub fn general_access_token(&self, sub: String) -> String {
        self.general_token(TokenKind::ACCESS, sub)
    }

    pub fn validate_access_token(&self, token: &str) -> Result<Claims, Error> {
        let result = self.validate_token(TokenKind::ACCESS, token)?;
        Ok(result.claims)
    }

    pub fn validate_refresh_token(&self, token: &str) -> Result<Claims, Error> {
        let result = self.validate_token(TokenKind::REFRESH, token)?;
        Ok(result.claims)
    }

    fn general_token(&self, kind: TokenKind, sub: String) -> String {
        let aud = self.aud.clone();
        let duration = self.duration.clone();
        let (iat, exp) = general_expired_time(duration);
        let (key, exp) = match kind {
            TokenKind::ACCESS => (&self.eak, exp),
            TokenKind::REFRESH => (&self.erk, 0),
        };
        let claims = Claims::new(aud, sub, exp, iat);
        encode(&self.header, &claims, key).unwrap()
    }

    fn validate_token(&self, kind: TokenKind, token: &str) -> Result<TokenData<Claims>, Error> {
        let (key, validation) = match kind {
            TokenKind::ACCESS => (&self.dak, &self.vak),
            TokenKind::REFRESH => (&self.drk, &self.vrk),
        };
        decode::<Claims>(token, key, validation)
    }
}

fn general_expired_time(duration: usize) -> (usize, usize) {
    let iat = Utc::now();
    let exp = Utc::now() + Duration::seconds(duration as i64);
    (iat.timestamp() as usize, exp.timestamp() as usize)
}

#[derive(Clone)]
pub struct Jwt {
    header: Header,
    eak: EncodingKey, // encoding_asseec_key
    erk: EncodingKey, // encoding_refresh_key
    dak: DecodingKey, // dencoding_asseec_key
    drk: DecodingKey, // dencoding_refresh_key
    vak: Validation,  // validation_of_access_token
    vrk: Validation,  // validation_of_refresh_token
    aud: String,
    duration: usize,
}

impl Jwt {
    pub fn new(cfg: JwtCfg) -> Self {
        let header = Header::default();
        let eak = EncodingKey::from_secret(cfg.access_secret.as_bytes());
        let erk = EncodingKey::from_secret(cfg.refresh_secret.as_bytes());
        let dak = DecodingKey::from_secret(cfg.access_secret.as_bytes());
        let drk = DecodingKey::from_secret(cfg.refresh_secret.as_bytes());
        let mut vak = Validation::default();
        vak.set_audience(&[cfg.aud.clone()]);
        let mut vrk = vak.clone();
        vrk.validate_exp = false;
        vrk.required_spec_claims.clear();
        let aud = cfg.aud;
        let duration = cfg.duration;
        Jwt {
            header,
            eak,
            dak,
            erk,
            drk,
            vak,
            vrk,
            aud,
            duration,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct JwtCfg {
    pub access_secret: String,
    pub refresh_secret: String,
    pub aud: String,
    pub duration: usize,
}
