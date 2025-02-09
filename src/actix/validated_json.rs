use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::future::Future;
use std::pin::Pin;
use std::{fmt, ops};
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

impl<T> ValidatedJson<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> ops::Deref for ValidatedJson<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for ValidatedJson<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: fmt::Display> fmt::Display for ValidatedJson<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<T: fmt::Debug> fmt::Debug for ValidatedJson<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<T: Serialize> Serialize for ValidatedJson<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<T> FromRequest for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + 'static,
{
    type Error = actix_web::Error;

    // Use a boxed future for the async implementation
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        // Get the future that will extract Json<T>
        let fut = web::Json::<T>::from_request(req, payload);

        // Box an async block that awaits the future
        Box::pin(async move {
            // Await the future to get a Json<T> instance
            let json_wrapper = fut.await?;

            // Validate the inner T value
            json_wrapper
                .validate()
                .map_err(actix_web::error::ErrorBadRequest)?;

            // Return the ValidatedJson wrapper containing the inner T value
            Ok(ValidatedJson(json_wrapper.into_inner()))
        })
    }
}
