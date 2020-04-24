use crate::codec::{write_cmd_to_buf, Encoder};
use bytes::BytesMut;

pub struct GeoAdd<'a>(&'a str, &'a str, &'a str, &'a str, &'a [&'a str]);

impl<'a> GeoAdd<'a> {
    pub fn with(
        key: &'a str,
        longitude: &'a str,
        latitude: &'a str,
        member: &'a str,
        members: &'a [&'a str],
    ) -> Self {
        if members.len() % 3 != 0 {
            panic!(
                "Additional members must be: longitude, latitude, member_name: {:?}",
                &members[..]
            );
        } else {
            GeoAdd(key, longitude, latitude, member, members)
        }
    }
}

impl<'a> Encoder for GeoAdd<'a> {
    fn encode(self, buf: &mut BytesMut) {
        write_cmd_to_buf(
            "GEOADD",
            &[&[self.0, self.1, self.2, self.3], self.4].concat(),
            buf,
        );
    }
}

pub struct GeoDist<'a>(&'a str, &'a str, &'a str, Option<&'a str>);

impl<'a> GeoDist<'a> {
    pub fn with(key: &'a str, member1: &'a str, member2: &'a str, unit: Option<&'a str>) -> Self {
        Self(key, member1, member2, unit)
    }
}

impl<'a> Encoder for GeoDist<'a> {
    fn encode(self, buf: &mut BytesMut) {
        if self.3.is_none() {
            write_cmd_to_buf("GEODIST", &[self.0, self.1, self.2], buf);
        } else {
            if !["m", "km", "ft", "mi"].contains(&self.3.unwrap()) {
                panic!("Units must be one of:  m, km, ft, mi. {}", self.3.unwrap());
            }
            write_cmd_to_buf("GEODIST", &[self.0, self.1, self.2, self.3.unwrap()], buf);
        }
    }
}

pub struct GeoHash<'a>(&'a str, &'a str, &'a [&'a str]);

impl<'a> GeoHash<'a> {
    pub fn with(key: &'a str, member: &'a str, members: &'a [&'a str]) -> Self {
        Self(key, member, members)
    }
}

impl<'a> Encoder for GeoHash<'a> {
    fn encode(self, buf: &mut BytesMut) {
        write_cmd_to_buf("GEOHASH", &[&[self.0, self.1], self.2].concat(), buf);
    }
}

pub struct GeoPos<'a>(&'a str, &'a str, &'a [&'a str]);

impl<'a> GeoPos<'a> {
    pub fn with(key: &'a str, member: &'a str, members: &'a [&'a str]) -> Self {
        Self(key, member, members)
    }
}

impl<'a> Encoder for GeoPos<'a> {
    fn encode(self, buf: &mut BytesMut) {
        if self.2.is_empty() {
            write_cmd_to_buf("GEOPOS", &[self.1], buf);
        } else {
            write_cmd_to_buf("GEOPOS", &[&[self.1], self.2].concat(), buf);
        }
    }
}

pub struct GeoRadius<'a>(&'a str, &'a str, &'a str, &'a str);

impl<'a> GeoRadius<'a> {
    pub fn with(key: &'a str, longitude: &'a str, latitude: &'a str, radius: &'a str) -> Self {
        if !["km", "m", "ft", "mi"].contains(&radius) {
            panic!("Units must be one of:  m, km, ft, mi. {}", radius);
        } else {
            Self(key, longitude, latitude, radius)
        }
    }
}

impl<'a> Encoder for GeoRadius<'a> {
    fn encode(self, buf: &mut BytesMut) {
        write_cmd_to_buf("GEORADIUS", &[self.0, self.1, self.2, self.3], buf);
    }
}
