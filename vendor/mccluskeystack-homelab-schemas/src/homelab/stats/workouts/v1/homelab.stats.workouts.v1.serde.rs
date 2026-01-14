// @generated
impl serde::Serialize for CardioStats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.total_workouts != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("homelab.stats.workouts.v1.CardioStats", len)?;
        if self.total_workouts != 0 {
            struct_ser.serialize_field("totalWorkouts", &self.total_workouts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CardioStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "total_workouts",
            "totalWorkouts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TotalWorkouts,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "totalWorkouts" | "total_workouts" => Ok(GeneratedField::TotalWorkouts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CardioStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.workouts.v1.CardioStats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CardioStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut total_workouts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TotalWorkouts => {
                            if total_workouts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalWorkouts"));
                            }
                            total_workouts__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CardioStats {
                    total_workouts: total_workouts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.workouts.v1.CardioStats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DateRangeStats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.date_range_start_at.is_some() {
            len += 1;
        }
        if self.date_range_end_at.is_some() {
            len += 1;
        }
        if self.total_workouts != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("homelab.stats.workouts.v1.DateRangeStats", len)?;
        if let Some(v) = self.date_range_start_at.as_ref() {
            struct_ser.serialize_field("dateRangeStartAt", v)?;
        }
        if let Some(v) = self.date_range_end_at.as_ref() {
            struct_ser.serialize_field("dateRangeEndAt", v)?;
        }
        if self.total_workouts != 0 {
            struct_ser.serialize_field("totalWorkouts", &self.total_workouts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DateRangeStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "date_range_start_at",
            "dateRangeStartAt",
            "date_range_end_at",
            "dateRangeEndAt",
            "total_workouts",
            "totalWorkouts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DateRangeStartAt,
            DateRangeEndAt,
            TotalWorkouts,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dateRangeStartAt" | "date_range_start_at" => Ok(GeneratedField::DateRangeStartAt),
                            "dateRangeEndAt" | "date_range_end_at" => Ok(GeneratedField::DateRangeEndAt),
                            "totalWorkouts" | "total_workouts" => Ok(GeneratedField::TotalWorkouts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DateRangeStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.workouts.v1.DateRangeStats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DateRangeStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut date_range_start_at__ = None;
                let mut date_range_end_at__ = None;
                let mut total_workouts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DateRangeStartAt => {
                            if date_range_start_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dateRangeStartAt"));
                            }
                            date_range_start_at__ = map_.next_value()?;
                        }
                        GeneratedField::DateRangeEndAt => {
                            if date_range_end_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dateRangeEndAt"));
                            }
                            date_range_end_at__ = map_.next_value()?;
                        }
                        GeneratedField::TotalWorkouts => {
                            if total_workouts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalWorkouts"));
                            }
                            total_workouts__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DateRangeStats {
                    date_range_start_at: date_range_start_at__,
                    date_range_end_at: date_range_end_at__,
                    total_workouts: total_workouts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.workouts.v1.DateRangeStats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetWorkoutsStatsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.snapshot_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("homelab.stats.workouts.v1.GetWorkoutsStatsRequest", len)?;
        if let Some(v) = self.snapshot_id.as_ref() {
            struct_ser.serialize_field("snapshotId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetWorkoutsStatsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "snapshot_id",
            "snapshotId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SnapshotId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "snapshotId" | "snapshot_id" => Ok(GeneratedField::SnapshotId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetWorkoutsStatsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.workouts.v1.GetWorkoutsStatsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetWorkoutsStatsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut snapshot_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SnapshotId => {
                            if snapshot_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshotId"));
                            }
                            snapshot_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetWorkoutsStatsRequest {
                    snapshot_id: snapshot_id__,
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.workouts.v1.GetWorkoutsStatsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetWorkoutsStatsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.workouts_stats.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("homelab.stats.workouts.v1.GetWorkoutsStatsResponse", len)?;
        if let Some(v) = self.workouts_stats.as_ref() {
            struct_ser.serialize_field("workoutsStats", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetWorkoutsStatsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workouts_stats",
            "workoutsStats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WorkoutsStats,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "workoutsStats" | "workouts_stats" => Ok(GeneratedField::WorkoutsStats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetWorkoutsStatsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.workouts.v1.GetWorkoutsStatsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetWorkoutsStatsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workouts_stats__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WorkoutsStats => {
                            if workouts_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workoutsStats"));
                            }
                            workouts_stats__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetWorkoutsStatsResponse {
                    workouts_stats: workouts_stats__,
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.workouts.v1.GetWorkoutsStatsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthCheckRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("homelab.stats.workouts.v1.HealthCheckRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheckRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HealthCheckRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.workouts.v1.HealthCheckRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HealthCheckRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HealthCheckRequest {
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.workouts.v1.HealthCheckRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthCheckResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        if !self.details.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("homelab.stats.workouts.v1.HealthCheckResponse", len)?;
        if self.status != 0 {
            let v = HealthCheckStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if !self.details.is_empty() {
            struct_ser.serialize_field("details", &self.details)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheckResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            Details,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "status" => Ok(GeneratedField::Status),
                            "details" => Ok(GeneratedField::Details),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HealthCheckResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.workouts.v1.HealthCheckResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HealthCheckResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<HealthCheckStatus>()? as i32);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(HealthCheckResponse {
                    status: status__.unwrap_or_default(),
                    details: details__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.workouts.v1.HealthCheckResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthCheckStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "HEALTH_CHECK_STATUS_UNSPECIFIED",
            Self::Up => "HEALTH_CHECK_STATUS_UP",
            Self::Down => "HEALTH_CHECK_STATUS_DOWN",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheckStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "HEALTH_CHECK_STATUS_UNSPECIFIED",
            "HEALTH_CHECK_STATUS_UP",
            "HEALTH_CHECK_STATUS_DOWN",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HealthCheckStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "HEALTH_CHECK_STATUS_UNSPECIFIED" => Ok(HealthCheckStatus::Unspecified),
                    "HEALTH_CHECK_STATUS_UP" => Ok(HealthCheckStatus::Up),
                    "HEALTH_CHECK_STATUS_DOWN" => Ok(HealthCheckStatus::Down),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for StrengthStats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.total_workouts != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("homelab.stats.workouts.v1.StrengthStats", len)?;
        if self.total_workouts != 0 {
            struct_ser.serialize_field("totalWorkouts", &self.total_workouts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StrengthStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "total_workouts",
            "totalWorkouts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TotalWorkouts,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "totalWorkouts" | "total_workouts" => Ok(GeneratedField::TotalWorkouts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StrengthStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.workouts.v1.StrengthStats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StrengthStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut total_workouts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TotalWorkouts => {
                            if total_workouts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalWorkouts"));
                            }
                            total_workouts__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(StrengthStats {
                    total_workouts: total_workouts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.workouts.v1.StrengthStats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UploadMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.filename.is_empty() {
            len += 1;
        }
        if self.size_bytes != 0 {
            len += 1;
        }
        if !self.checksum.is_empty() {
            len += 1;
        }
        if self.file_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("homelab.stats.workouts.v1.UploadMetadata", len)?;
        if !self.filename.is_empty() {
            struct_ser.serialize_field("filename", &self.filename)?;
        }
        if self.size_bytes != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("sizeBytes", ToString::to_string(&self.size_bytes).as_str())?;
        }
        if !self.checksum.is_empty() {
            struct_ser.serialize_field("checksum", &self.checksum)?;
        }
        if self.file_type != 0 {
            let v = UploadWorkoutsFileType::try_from(self.file_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.file_type)))?;
            struct_ser.serialize_field("fileType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UploadMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filename",
            "size_bytes",
            "sizeBytes",
            "checksum",
            "file_type",
            "fileType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Filename,
            SizeBytes,
            Checksum,
            FileType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "filename" => Ok(GeneratedField::Filename),
                            "sizeBytes" | "size_bytes" => Ok(GeneratedField::SizeBytes),
                            "checksum" => Ok(GeneratedField::Checksum),
                            "fileType" | "file_type" => Ok(GeneratedField::FileType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UploadMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.workouts.v1.UploadMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UploadMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filename__ = None;
                let mut size_bytes__ = None;
                let mut checksum__ = None;
                let mut file_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Filename => {
                            if filename__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filename"));
                            }
                            filename__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SizeBytes => {
                            if size_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sizeBytes"));
                            }
                            size_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Checksum => {
                            if checksum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checksum"));
                            }
                            checksum__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FileType => {
                            if file_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileType"));
                            }
                            file_type__ = Some(map_.next_value::<UploadWorkoutsFileType>()? as i32);
                        }
                    }
                }
                Ok(UploadMetadata {
                    filename: filename__.unwrap_or_default(),
                    size_bytes: size_bytes__.unwrap_or_default(),
                    checksum: checksum__.unwrap_or_default(),
                    file_type: file_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.workouts.v1.UploadMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UploadWorkoutsError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if !self.details.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("homelab.stats.workouts.v1.UploadWorkoutsError", len)?;
        if self.code != 0 {
            let v = UploadWorkoutsErrorCode::try_from(self.code)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.code)))?;
            struct_ser.serialize_field("code", &v)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if !self.details.is_empty() {
            struct_ser.serialize_field("details", &self.details)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UploadWorkoutsError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "message",
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Message,
            Details,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "message" => Ok(GeneratedField::Message),
                            "details" => Ok(GeneratedField::Details),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UploadWorkoutsError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.workouts.v1.UploadWorkoutsError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UploadWorkoutsError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut message__ = None;
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value::<UploadWorkoutsErrorCode>()? as i32);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(UploadWorkoutsError {
                    code: code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    details: details__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.workouts.v1.UploadWorkoutsError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UploadWorkoutsErrorCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::ErrorTypeUnspecified => "UPLOAD_WORKOUTS_ERROR_CODE_ERROR_TYPE_UNSPECIFIED",
            Self::InvalidSchema => "UPLOAD_WORKOUTS_ERROR_CODE_INVALID_SCHEMA",
            Self::CorruptedFile => "UPLOAD_WORKOUTS_ERROR_CODE_CORRUPTED_FILE",
            Self::ChecksumMismatch => "UPLOAD_WORKOUTS_ERROR_CODE_CHECKSUM_MISMATCH",
            Self::DuplicateData => "UPLOAD_WORKOUTS_ERROR_CODE_DUPLICATE_DATA",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for UploadWorkoutsErrorCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UPLOAD_WORKOUTS_ERROR_CODE_ERROR_TYPE_UNSPECIFIED",
            "UPLOAD_WORKOUTS_ERROR_CODE_INVALID_SCHEMA",
            "UPLOAD_WORKOUTS_ERROR_CODE_CORRUPTED_FILE",
            "UPLOAD_WORKOUTS_ERROR_CODE_CHECKSUM_MISMATCH",
            "UPLOAD_WORKOUTS_ERROR_CODE_DUPLICATE_DATA",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UploadWorkoutsErrorCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UPLOAD_WORKOUTS_ERROR_CODE_ERROR_TYPE_UNSPECIFIED" => Ok(UploadWorkoutsErrorCode::ErrorTypeUnspecified),
                    "UPLOAD_WORKOUTS_ERROR_CODE_INVALID_SCHEMA" => Ok(UploadWorkoutsErrorCode::InvalidSchema),
                    "UPLOAD_WORKOUTS_ERROR_CODE_CORRUPTED_FILE" => Ok(UploadWorkoutsErrorCode::CorruptedFile),
                    "UPLOAD_WORKOUTS_ERROR_CODE_CHECKSUM_MISMATCH" => Ok(UploadWorkoutsErrorCode::ChecksumMismatch),
                    "UPLOAD_WORKOUTS_ERROR_CODE_DUPLICATE_DATA" => Ok(UploadWorkoutsErrorCode::DuplicateData),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UploadWorkoutsFileType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UPLOAD_WORKOUTS_FILE_TYPE_UNSPECIFIED",
            Self::Sqlite => "UPLOAD_WORKOUTS_FILE_TYPE_SQLITE",
            Self::Csv => "UPLOAD_WORKOUTS_FILE_TYPE_CSV",
            Self::Json => "UPLOAD_WORKOUTS_FILE_TYPE_JSON",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for UploadWorkoutsFileType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UPLOAD_WORKOUTS_FILE_TYPE_UNSPECIFIED",
            "UPLOAD_WORKOUTS_FILE_TYPE_SQLITE",
            "UPLOAD_WORKOUTS_FILE_TYPE_CSV",
            "UPLOAD_WORKOUTS_FILE_TYPE_JSON",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UploadWorkoutsFileType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UPLOAD_WORKOUTS_FILE_TYPE_UNSPECIFIED" => Ok(UploadWorkoutsFileType::Unspecified),
                    "UPLOAD_WORKOUTS_FILE_TYPE_SQLITE" => Ok(UploadWorkoutsFileType::Sqlite),
                    "UPLOAD_WORKOUTS_FILE_TYPE_CSV" => Ok(UploadWorkoutsFileType::Csv),
                    "UPLOAD_WORKOUTS_FILE_TYPE_JSON" => Ok(UploadWorkoutsFileType::Json),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UploadWorkoutsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("homelab.stats.workouts.v1.UploadWorkoutsRequest", len)?;
        if let Some(v) = self.data.as_ref() {
            match v {
                upload_workouts_request::Data::Metadata(v) => {
                    struct_ser.serialize_field("metadata", v)?;
                }
                upload_workouts_request::Data::Chunk(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("chunk", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UploadWorkoutsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "chunk",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Chunk,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "metadata" => Ok(GeneratedField::Metadata),
                            "chunk" => Ok(GeneratedField::Chunk),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UploadWorkoutsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.workouts.v1.UploadWorkoutsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UploadWorkoutsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(upload_workouts_request::Data::Metadata)
;
                        }
                        GeneratedField::Chunk => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunk"));
                            }
                            data__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| upload_workouts_request::Data::Chunk(x.0));
                        }
                    }
                }
                Ok(UploadWorkoutsRequest {
                    data: data__,
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.workouts.v1.UploadWorkoutsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UploadWorkoutsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        if self.bytes_received != 0 {
            len += 1;
        }
        if self.workouts_processed != 0 {
            len += 1;
        }
        if !self.warnings.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("homelab.stats.workouts.v1.UploadWorkoutsResponse", len)?;
        if self.status != 0 {
            let v = UploadWorkoutsStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if self.bytes_received != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("bytesReceived", ToString::to_string(&self.bytes_received).as_str())?;
        }
        if self.workouts_processed != 0 {
            struct_ser.serialize_field("workoutsProcessed", &self.workouts_processed)?;
        }
        if !self.warnings.is_empty() {
            struct_ser.serialize_field("warnings", &self.warnings)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UploadWorkoutsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "bytes_received",
            "bytesReceived",
            "workouts_processed",
            "workoutsProcessed",
            "warnings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            BytesReceived,
            WorkoutsProcessed,
            Warnings,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "status" => Ok(GeneratedField::Status),
                            "bytesReceived" | "bytes_received" => Ok(GeneratedField::BytesReceived),
                            "workoutsProcessed" | "workouts_processed" => Ok(GeneratedField::WorkoutsProcessed),
                            "warnings" => Ok(GeneratedField::Warnings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UploadWorkoutsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.workouts.v1.UploadWorkoutsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UploadWorkoutsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut bytes_received__ = None;
                let mut workouts_processed__ = None;
                let mut warnings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<UploadWorkoutsStatus>()? as i32);
                        }
                        GeneratedField::BytesReceived => {
                            if bytes_received__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bytesReceived"));
                            }
                            bytes_received__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::WorkoutsProcessed => {
                            if workouts_processed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workoutsProcessed"));
                            }
                            workouts_processed__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Warnings => {
                            if warnings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warnings"));
                            }
                            warnings__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UploadWorkoutsResponse {
                    status: status__.unwrap_or_default(),
                    bytes_received: bytes_received__.unwrap_or_default(),
                    workouts_processed: workouts_processed__.unwrap_or_default(),
                    warnings: warnings__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.workouts.v1.UploadWorkoutsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UploadWorkoutsStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UPLOAD_WORKOUTS_STATUS_UNSPECIFIED",
            Self::Processing => "UPLOAD_WORKOUTS_STATUS_PROCESSING",
            Self::Completed => "UPLOAD_WORKOUTS_STATUS_COMPLETED",
            Self::Failed => "UPLOAD_WORKOUTS_STATUS_FAILED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for UploadWorkoutsStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UPLOAD_WORKOUTS_STATUS_UNSPECIFIED",
            "UPLOAD_WORKOUTS_STATUS_PROCESSING",
            "UPLOAD_WORKOUTS_STATUS_COMPLETED",
            "UPLOAD_WORKOUTS_STATUS_FAILED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UploadWorkoutsStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UPLOAD_WORKOUTS_STATUS_UNSPECIFIED" => Ok(UploadWorkoutsStatus::Unspecified),
                    "UPLOAD_WORKOUTS_STATUS_PROCESSING" => Ok(UploadWorkoutsStatus::Processing),
                    "UPLOAD_WORKOUTS_STATUS_COMPLETED" => Ok(UploadWorkoutsStatus::Completed),
                    "UPLOAD_WORKOUTS_STATUS_FAILED" => Ok(UploadWorkoutsStatus::Failed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for WorkoutsStats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.snapshot_at.is_some() {
            len += 1;
        }
        if !self.snapshot_id.is_empty() {
            len += 1;
        }
        if self.total_workouts != 0 {
            len += 1;
        }
        if self.total_workout_time_ms != 0 {
            len += 1;
        }
        if self.last_week_stats.is_some() {
            len += 1;
        }
        if self.last_month_stats.is_some() {
            len += 1;
        }
        if self.last_year_stats.is_some() {
            len += 1;
        }
        if self.strength_stats.is_some() {
            len += 1;
        }
        if self.cardio_stats.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("homelab.stats.workouts.v1.WorkoutsStats", len)?;
        if let Some(v) = self.snapshot_at.as_ref() {
            struct_ser.serialize_field("snapshotAt", v)?;
        }
        if !self.snapshot_id.is_empty() {
            struct_ser.serialize_field("snapshotId", &self.snapshot_id)?;
        }
        if self.total_workouts != 0 {
            struct_ser.serialize_field("totalWorkouts", &self.total_workouts)?;
        }
        if self.total_workout_time_ms != 0 {
            struct_ser.serialize_field("totalWorkoutTimeMs", &self.total_workout_time_ms)?;
        }
        if let Some(v) = self.last_week_stats.as_ref() {
            struct_ser.serialize_field("lastWeekStats", v)?;
        }
        if let Some(v) = self.last_month_stats.as_ref() {
            struct_ser.serialize_field("lastMonthStats", v)?;
        }
        if let Some(v) = self.last_year_stats.as_ref() {
            struct_ser.serialize_field("lastYearStats", v)?;
        }
        if let Some(v) = self.strength_stats.as_ref() {
            struct_ser.serialize_field("strengthStats", v)?;
        }
        if let Some(v) = self.cardio_stats.as_ref() {
            struct_ser.serialize_field("cardioStats", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WorkoutsStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "snapshot_at",
            "snapshotAt",
            "snapshot_id",
            "snapshotId",
            "total_workouts",
            "totalWorkouts",
            "total_workout_time_ms",
            "totalWorkoutTimeMs",
            "last_week_stats",
            "lastWeekStats",
            "last_month_stats",
            "lastMonthStats",
            "last_year_stats",
            "lastYearStats",
            "strength_stats",
            "strengthStats",
            "cardio_stats",
            "cardioStats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SnapshotAt,
            SnapshotId,
            TotalWorkouts,
            TotalWorkoutTimeMs,
            LastWeekStats,
            LastMonthStats,
            LastYearStats,
            StrengthStats,
            CardioStats,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "snapshotAt" | "snapshot_at" => Ok(GeneratedField::SnapshotAt),
                            "snapshotId" | "snapshot_id" => Ok(GeneratedField::SnapshotId),
                            "totalWorkouts" | "total_workouts" => Ok(GeneratedField::TotalWorkouts),
                            "totalWorkoutTimeMs" | "total_workout_time_ms" => Ok(GeneratedField::TotalWorkoutTimeMs),
                            "lastWeekStats" | "last_week_stats" => Ok(GeneratedField::LastWeekStats),
                            "lastMonthStats" | "last_month_stats" => Ok(GeneratedField::LastMonthStats),
                            "lastYearStats" | "last_year_stats" => Ok(GeneratedField::LastYearStats),
                            "strengthStats" | "strength_stats" => Ok(GeneratedField::StrengthStats),
                            "cardioStats" | "cardio_stats" => Ok(GeneratedField::CardioStats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WorkoutsStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.workouts.v1.WorkoutsStats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WorkoutsStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut snapshot_at__ = None;
                let mut snapshot_id__ = None;
                let mut total_workouts__ = None;
                let mut total_workout_time_ms__ = None;
                let mut last_week_stats__ = None;
                let mut last_month_stats__ = None;
                let mut last_year_stats__ = None;
                let mut strength_stats__ = None;
                let mut cardio_stats__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SnapshotAt => {
                            if snapshot_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshotAt"));
                            }
                            snapshot_at__ = map_.next_value()?;
                        }
                        GeneratedField::SnapshotId => {
                            if snapshot_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshotId"));
                            }
                            snapshot_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalWorkouts => {
                            if total_workouts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalWorkouts"));
                            }
                            total_workouts__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalWorkoutTimeMs => {
                            if total_workout_time_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalWorkoutTimeMs"));
                            }
                            total_workout_time_ms__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LastWeekStats => {
                            if last_week_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastWeekStats"));
                            }
                            last_week_stats__ = map_.next_value()?;
                        }
                        GeneratedField::LastMonthStats => {
                            if last_month_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastMonthStats"));
                            }
                            last_month_stats__ = map_.next_value()?;
                        }
                        GeneratedField::LastYearStats => {
                            if last_year_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastYearStats"));
                            }
                            last_year_stats__ = map_.next_value()?;
                        }
                        GeneratedField::StrengthStats => {
                            if strength_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("strengthStats"));
                            }
                            strength_stats__ = map_.next_value()?;
                        }
                        GeneratedField::CardioStats => {
                            if cardio_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cardioStats"));
                            }
                            cardio_stats__ = map_.next_value()?;
                        }
                    }
                }
                Ok(WorkoutsStats {
                    snapshot_at: snapshot_at__,
                    snapshot_id: snapshot_id__.unwrap_or_default(),
                    total_workouts: total_workouts__.unwrap_or_default(),
                    total_workout_time_ms: total_workout_time_ms__.unwrap_or_default(),
                    last_week_stats: last_week_stats__,
                    last_month_stats: last_month_stats__,
                    last_year_stats: last_year_stats__,
                    strength_stats: strength_stats__,
                    cardio_stats: cardio_stats__,
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.workouts.v1.WorkoutsStats", FIELDS, GeneratedVisitor)
    }
}
