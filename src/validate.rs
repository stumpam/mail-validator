use actix_web::{get, HttpRequest, HttpResponse, Responder};
use qstring::QString;
use regex::Regex;
use serde::Serialize;

mod tlds;

use tlds::TLDS;

#[derive(Serialize)]
struct Response {
    valid: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ErrorResponse {
    error: &'static str,
    error_code: i8,
}

#[get("/validate_email")]
async fn validate(req: HttpRequest) -> impl Responder {
    let qs = QString::from(req.query_string());
    let param = qs.get("email");

    match param {
        Some(email) => HttpResponse::Ok().json(Response {
            valid: validate_email(email),
        }),
        None => HttpResponse::BadRequest().json(ErrorResponse {
            error: "Query param email was not send",
            error_code: 1,
        }),
    }
}

fn validate_email(email: &str) -> bool {
    let email_regex = Regex::new("[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?").unwrap();

    let email_is_valid = email_regex.is_match(email);
    let tld = email.split('.').last().unwrap_or("");
    let email_has_valid_tld = validate_tld(tld);

    email_is_valid && email_has_valid_tld
}

fn validate_tld(tld: &str) -> bool {
    if tld == "" {
        return false;
    }

    TLDS.contains(&tld.to_uppercase().as_str())
}

#[cfg(test)]
mod tests {
    #[test]
    fn validate_email() {
        const CASES: &'static [(&str, bool)] = &[
            ("test.aaaa@test.com.ee", true),
            ("test.aaaa@test.com", true),
            ("test@test.com", true),
            ("abc@bc.ttttt", false),
        ];

        let test = CASES
            .iter()
            .all(|(email, result)| super::validate_email(email) == *result);

        assert_eq!(test, true);
    }
}
