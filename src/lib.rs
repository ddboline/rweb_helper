pub mod content_type_trait;
pub mod html_response;
pub mod json_response;
pub mod response_description_trait;
pub mod status_code_trait;

pub use rweb_helper_macro::RwebResponse;

#[macro_export]
macro_rules! derive_rweb_schema {
    ($T0:ty, $T1:ty) => {
        impl rweb::openapi::Entity for $T0 {
            fn type_name() -> std::borrow::Cow<'static, str> {
                assert_eq!(std::mem::size_of::<$T0>(), std::mem::size_of::<$T1>());
                <$T1>::type_name()
            }
            fn describe(c: &mut rweb::openapi::ComponentDescriptor) -> rweb::openapi::ComponentOrInlineSchema {
                <$T1>::describe(c)
            }
        }
    }
}

#[macro_export]
macro_rules! derive_rweb_test {
    ($T0:ty, $T1:ty) => {
        assert_eq!(std::mem::size_of::<$T0>(), std::mem::size_of::<$T1>());
    };
}
