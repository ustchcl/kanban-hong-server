use actix_web::http::StatusCode;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug)]
pub struct Errors {
  errors: ValidationErrors,
}

pub type FieldName = &'static str;
pub type FieldErrorCode = &'static str;

impl Errors {
  pub fn new(errs: &[(FieldName, FieldErrorCode)]) -> Self {
      let mut errors = ValidationErrors::new();
      for (field, code) in errs {
          errors.add(field, ValidationError::new(code));
      }
      Self { errors }
  }
}

impl<'r> Responder<'r> for Errors {
  fn respond_to(self, req: &Request) -> response::Result<'r> {
      use validator::ValidationErrorsKind::Field;

      let mut errors = json!({});
      for (field, field_errors) in self.errors.into_errors() {
          if let Field(field_errors) = field_errors {
              errors[field] = field_errors.into_iter().map(|field_error| field_error.code).collect();
          }
      }

      status::Custom(
          Status::UnprocessableEntity,
          Json(json!({ "errors": errors })),
      )
      .respond_to(req)
  }
}