//! Transport from Dummy Source to Arrow Destination.

use crate::destinations::arrow::{ArrowDestination, ArrowDestinationError, ArrowTypeSystem};
use crate::sources::dummy::{DummySource, DummyTypeSystem};
use crate::typesystem::TypeConversion;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use thiserror::Error;

/// Convert Dummy data types to Arrow data types.
pub struct DummyArrowTransport;

#[derive(Error, Debug)]
pub enum DummyArrowTransportError {
    #[error(transparent)]
    Destination(#[from] ArrowDestinationError),

    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

impl_transport!(
    name = DummyArrowTransport,
    error = DummyArrowTransportError,
    systems = DummyTypeSystem => ArrowTypeSystem,
    route = DummySource => ArrowDestination,
    mappings = {
        { F64[f64]                => Float64[f64]               | conversion auto}
        { I64[i64]                => Int64[i64]                 | conversion auto}
        { Bool[bool]              => Boolean[bool]              | conversion auto}
        { String[String]          => LargeUtf8[String]          | conversion auto}
        { DateTime[DateTime<Utc>] => Date64[NaiveDateTime]      | conversion option}
    }
);

impl TypeConversion<DateTime<Utc>, NaiveDateTime> for DummyArrowTransport {
    fn convert(val: DateTime<Utc>) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(val.timestamp(), val.timestamp_subsec_nanos())
    }
}

impl TypeConversion<NaiveDateTime, DateTime<Utc>> for DummyArrowTransport {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl TypeConversion<NaiveDate, DateTime<Utc>> for DummyArrowTransport {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}
