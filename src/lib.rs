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
            fn describe(
                c: &mut rweb::openapi::ComponentDescriptor,
            ) -> rweb::openapi::ComponentOrInlineSchema {
                <$T1>::describe(c)
            }
        }
    };
}

#[macro_export]
macro_rules! derive_rweb_test {
    ($T0:ty, $T1:ty) => {
        assert_eq!(std::mem::size_of::<$T0>(), std::mem::size_of::<$T1>());
    };
}

use derive_more::{Deref, Display, From, FromStr, Into};
use rust_decimal::Decimal;
use rweb::openapi::{ComponentDescriptor, ComponentOrInlineSchema, Entity, Schema, Type};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use time::{Date, OffsetDateTime};
use uuid::Uuid;

#[derive(Into, From, Serialize, Deserialize, Deref, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DateTimeType(OffsetDateTime);

impl Entity for DateTimeType {
    fn type_name() -> Cow<'static, str> {
        Cow::Borrowed("date-time")
    }

    fn describe(_: &mut ComponentDescriptor) -> ComponentOrInlineSchema {
        ComponentOrInlineSchema::Inline(Schema {
            schema_type: Some(Type::String),
            format: Self::type_name(),
            ..Schema::default()
        })
    }
}

#[derive(Into, From, Serialize, Deserialize, Deref, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct DateType(Date);

impl Entity for DateType {
    fn type_name() -> Cow<'static, str> {
        Cow::Borrowed("date")
    }

    fn describe(_: &mut ComponentDescriptor) -> ComponentOrInlineSchema {
        ComponentOrInlineSchema::Inline(Schema {
            schema_type: Some(Type::String),
            format: Self::type_name(),
            ..Schema::default()
        })
    }
}

#[derive(
    Into,
    From,
    Serialize,
    Deserialize,
    Deref,
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    FromStr,
    Display,
)]
pub struct UuidWrapper(Uuid);

impl Entity for UuidWrapper {
    fn type_name() -> Cow<'static, str> {
        Cow::Borrowed("uuid")
    }

    fn describe(_: &mut ComponentDescriptor) -> ComponentOrInlineSchema {
        ComponentOrInlineSchema::Inline(Schema {
            schema_type: Some(Type::String),
            format: Self::type_name(),
            ..Default::default()
        })
    }
}

impl PartialEq<Uuid> for UuidWrapper {
    fn eq(&self, other: &Uuid) -> bool {
        &self.0 == other
    }
}

impl PartialEq<UuidWrapper> for Uuid {
    fn eq(&self, other: &UuidWrapper) -> bool {
        self == &other.0
    }
}

#[derive(
    Into,
    From,
    Serialize,
    Deserialize,
    Deref,
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    FromStr,
    Display,
)]
pub struct DecimalWrapper(Decimal);

impl Entity for DecimalWrapper {
    fn type_name() -> Cow<'static, str> {
        Cow::Borrowed("decimal")
    }

    fn describe(_: &mut ComponentDescriptor) -> ComponentOrInlineSchema {
        ComponentOrInlineSchema::Inline(Schema {
            schema_type: Some(Type::String),
            format: Self::type_name(),
            ..Default::default()
        })
    }
}

impl PartialEq<Decimal> for DecimalWrapper {
    fn eq(&self, other: &Decimal) -> bool {
        &self.0 == other
    }
}

impl PartialEq<DecimalWrapper> for Decimal {
    fn eq(&self, other: &DecimalWrapper) -> bool {
        self == &other.0
    }
}
