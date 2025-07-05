use std::{borrow::Cow, sync::LazyLock, time::Duration};

use jsonwebtoken::{ encode, decode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use anyhow::Result;

const DEFAULT_SECRET: &str = "secret";

static DEFAULT_JWT: LazyLock<JWT> = LazyLock::new(|| JWT::default());

// jwt 中的主体
#[derive(Debug, Clone)]
pub struct Principal {
    pub id: String,
    pub name: String,
}

// jwt 中的声明
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // jwt id (标识)
    pub jti: String,
    // subject (主题)
    pub sub: String,
    // audience (受众)
    pub aud: String,
    // issuer (颁发者)
    pub iss: String,
    // expiration time (过期时间)
    pub exp: u64,
    // issuer at time (颁发时间)
    pub iat: u64,
}

#[derive(Debug)]
pub struct JwtConfig {
    // 秘钥
    pub secret: Cow<'static, str>,
    // 发行人
    pub audience: String,
    // 颁发者
    pub issuer: String,
    // 过期时间
    pub expiration: Duration,
}

impl Default for JwtConfig {
    fn default() -> Self {
        JwtConfig {
            secret: Cow::Borrowed(DEFAULT_SECRET),
            audience: String::from("audience"),
            issuer: String::from("issuer"),
            expiration: Duration::from_secs(60 * 60),
        }
    }
}

pub struct JWT {
    // 秘钥jwt 结构
    encode_secret: EncodingKey,
    decode_secret: DecodingKey,

    header: Header,

    validation: Validation,

    expiration: Duration,

    audience: String,

    issuer: String,
}

impl JWT {
    pub fn new(config: JwtConfig) -> Self {
        let secret = config.secret.as_bytes();
        let header = Header::new(Algorithm::HS256);
        // 在解析时候需要用到
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[&config.audience]);
        //
        validation.set_issuer(&[&config.issuer]);
        // 设置必须的字段
        validation.set_required_spec_claims(&["jti", "sub", "aud", "iss", "iat", "exp"]);

        Self {
            encode_secret: EncodingKey::from_secret(secret),
            decode_secret: DecodingKey::from_secret(secret),
            header,
            validation,
            expiration: config.expiration,
            audience: config.audience,
            issuer: config.issuer,
        }
    }

    pub fn encode(&self, principal: Principal) -> Result<String> {
        let now = get_current_timestamp();
        let claims = Claims {
            jti: xid::new().to_string(),
            sub: format!("{}:{}", principal.id, principal.name), //principal.id.clone(),
            aud: self.audience.clone(),
            iss: self.issuer.clone(),
            exp: now.saturating_add(self.expiration.as_secs()),
            iat: now,
        };
        Ok(
            encode(&self.header, &claims, &self.encode_secret)?
        )
    }

    pub fn decode(&self, token: &str) -> Result<Principal> { 
        let claims: Claims = decode(token, &self.decode_secret, &self.validation)?.claims;
        let mut parts = claims.sub.splitn(2, ":");
        let principal = Principal {
            id: parts.next().unwrap().to_string(),
            name: parts.next().unwrap().to_string(),
        };
        Ok(principal)
    }
}

impl Default for JWT {
    fn default() -> Self {
        Self::new(JwtConfig::default())
    }
}

pub fn get_jwt() -> &'static JWT {
    &DEFAULT_JWT
}
