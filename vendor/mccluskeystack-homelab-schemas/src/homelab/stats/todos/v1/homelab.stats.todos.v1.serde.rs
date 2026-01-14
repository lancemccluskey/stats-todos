// @generated
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
        if self.stats.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("homelab.stats.todos.v1.DateRangeStats", len)?;
        if let Some(v) = self.date_range_start_at.as_ref() {
            struct_ser.serialize_field("dateRangeStartAt", v)?;
        }
        if let Some(v) = self.date_range_end_at.as_ref() {
            struct_ser.serialize_field("dateRangeEndAt", v)?;
        }
        if let Some(v) = self.stats.as_ref() {
            struct_ser.serialize_field("stats", v)?;
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
            "stats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DateRangeStartAt,
            DateRangeEndAt,
            Stats,
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
                            "stats" => Ok(GeneratedField::Stats),
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
                formatter.write_str("struct homelab.stats.todos.v1.DateRangeStats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DateRangeStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut date_range_start_at__ = None;
                let mut date_range_end_at__ = None;
                let mut stats__ = None;
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
                        GeneratedField::Stats => {
                            if stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            stats__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DateRangeStats {
                    date_range_start_at: date_range_start_at__,
                    date_range_end_at: date_range_end_at__,
                    stats: stats__,
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.todos.v1.DateRangeStats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTodosStatsRequest {
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
        let mut struct_ser = serializer.serialize_struct("homelab.stats.todos.v1.GetTodosStatsRequest", len)?;
        if let Some(v) = self.snapshot_id.as_ref() {
            struct_ser.serialize_field("snapshotId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTodosStatsRequest {
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
            type Value = GetTodosStatsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.todos.v1.GetTodosStatsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTodosStatsRequest, V::Error>
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
                Ok(GetTodosStatsRequest {
                    snapshot_id: snapshot_id__,
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.todos.v1.GetTodosStatsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTodosStatsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.todos_stats.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("homelab.stats.todos.v1.GetTodosStatsResponse", len)?;
        if let Some(v) = self.todos_stats.as_ref() {
            struct_ser.serialize_field("todosStats", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTodosStatsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "todos_stats",
            "todosStats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TodosStats,
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
                            "todosStats" | "todos_stats" => Ok(GeneratedField::TodosStats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTodosStatsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.todos.v1.GetTodosStatsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTodosStatsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut todos_stats__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TodosStats => {
                            if todos_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("todosStats"));
                            }
                            todos_stats__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetTodosStatsResponse {
                    todos_stats: todos_stats__,
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.todos.v1.GetTodosStatsResponse", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("homelab.stats.todos.v1.HealthCheckRequest", len)?;
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
                formatter.write_str("struct homelab.stats.todos.v1.HealthCheckRequest")
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
        deserializer.deserialize_struct("homelab.stats.todos.v1.HealthCheckRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("homelab.stats.todos.v1.HealthCheckResponse", len)?;
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
                formatter.write_str("struct homelab.stats.todos.v1.HealthCheckResponse")
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
        deserializer.deserialize_struct("homelab.stats.todos.v1.HealthCheckResponse", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for ProjectStats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project.is_empty() {
            len += 1;
        }
        if self.stats.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("homelab.stats.todos.v1.ProjectStats", len)?;
        if !self.project.is_empty() {
            struct_ser.serialize_field("project", &self.project)?;
        }
        if let Some(v) = self.stats.as_ref() {
            struct_ser.serialize_field("stats", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProjectStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project",
            "stats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Project,
            Stats,
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
                            "project" => Ok(GeneratedField::Project),
                            "stats" => Ok(GeneratedField::Stats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProjectStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.todos.v1.ProjectStats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProjectStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project__ = None;
                let mut stats__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Project => {
                            if project__.is_some() {
                                return Err(serde::de::Error::duplicate_field("project"));
                            }
                            project__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Stats => {
                            if stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            stats__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ProjectStats {
                    project: project__.unwrap_or_default(),
                    stats: stats__,
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.todos.v1.ProjectStats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Stats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.completed != 0 {
            len += 1;
        }
        if self.created != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("homelab.stats.todos.v1.Stats", len)?;
        if self.completed != 0 {
            struct_ser.serialize_field("completed", &self.completed)?;
        }
        if self.created != 0 {
            struct_ser.serialize_field("created", &self.created)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Stats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "completed",
            "created",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Completed,
            Created,
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
                            "completed" => Ok(GeneratedField::Completed),
                            "created" => Ok(GeneratedField::Created),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Stats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.todos.v1.Stats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Stats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut completed__ = None;
                let mut created__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Completed => {
                            if completed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("completed"));
                            }
                            completed__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Created => {
                            if created__.is_some() {
                                return Err(serde::de::Error::duplicate_field("created"));
                            }
                            created__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Stats {
                    completed: completed__.unwrap_or_default(),
                    created: created__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.todos.v1.Stats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TagStats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tag.is_empty() {
            len += 1;
        }
        if self.stats.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("homelab.stats.todos.v1.TagStats", len)?;
        if !self.tag.is_empty() {
            struct_ser.serialize_field("tag", &self.tag)?;
        }
        if let Some(v) = self.stats.as_ref() {
            struct_ser.serialize_field("stats", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TagStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tag",
            "stats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tag,
            Stats,
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
                            "tag" => Ok(GeneratedField::Tag),
                            "stats" => Ok(GeneratedField::Stats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TagStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.todos.v1.TagStats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TagStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tag__ = None;
                let mut stats__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tag => {
                            if tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            tag__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Stats => {
                            if stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            stats__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TagStats {
                    tag: tag__.unwrap_or_default(),
                    stats: stats__,
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.todos.v1.TagStats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TodosStats {
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
        if self.total_todos != 0 {
            len += 1;
        }
        if self.completed_todos != 0 {
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
        if !self.tag_stats.is_empty() {
            len += 1;
        }
        if !self.project_stats.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("homelab.stats.todos.v1.TodosStats", len)?;
        if let Some(v) = self.snapshot_at.as_ref() {
            struct_ser.serialize_field("snapshotAt", v)?;
        }
        if !self.snapshot_id.is_empty() {
            struct_ser.serialize_field("snapshotId", &self.snapshot_id)?;
        }
        if self.total_todos != 0 {
            struct_ser.serialize_field("totalTodos", &self.total_todos)?;
        }
        if self.completed_todos != 0 {
            struct_ser.serialize_field("completedTodos", &self.completed_todos)?;
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
        if !self.tag_stats.is_empty() {
            struct_ser.serialize_field("tagStats", &self.tag_stats)?;
        }
        if !self.project_stats.is_empty() {
            struct_ser.serialize_field("projectStats", &self.project_stats)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TodosStats {
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
            "total_todos",
            "totalTodos",
            "completed_todos",
            "completedTodos",
            "last_week_stats",
            "lastWeekStats",
            "last_month_stats",
            "lastMonthStats",
            "last_year_stats",
            "lastYearStats",
            "tag_stats",
            "tagStats",
            "project_stats",
            "projectStats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SnapshotAt,
            SnapshotId,
            TotalTodos,
            CompletedTodos,
            LastWeekStats,
            LastMonthStats,
            LastYearStats,
            TagStats,
            ProjectStats,
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
                            "totalTodos" | "total_todos" => Ok(GeneratedField::TotalTodos),
                            "completedTodos" | "completed_todos" => Ok(GeneratedField::CompletedTodos),
                            "lastWeekStats" | "last_week_stats" => Ok(GeneratedField::LastWeekStats),
                            "lastMonthStats" | "last_month_stats" => Ok(GeneratedField::LastMonthStats),
                            "lastYearStats" | "last_year_stats" => Ok(GeneratedField::LastYearStats),
                            "tagStats" | "tag_stats" => Ok(GeneratedField::TagStats),
                            "projectStats" | "project_stats" => Ok(GeneratedField::ProjectStats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TodosStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.todos.v1.TodosStats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TodosStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut snapshot_at__ = None;
                let mut snapshot_id__ = None;
                let mut total_todos__ = None;
                let mut completed_todos__ = None;
                let mut last_week_stats__ = None;
                let mut last_month_stats__ = None;
                let mut last_year_stats__ = None;
                let mut tag_stats__ = None;
                let mut project_stats__ = None;
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
                        GeneratedField::TotalTodos => {
                            if total_todos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalTodos"));
                            }
                            total_todos__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CompletedTodos => {
                            if completed_todos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("completedTodos"));
                            }
                            completed_todos__ = 
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
                        GeneratedField::TagStats => {
                            if tag_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagStats"));
                            }
                            tag_stats__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectStats => {
                            if project_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectStats"));
                            }
                            project_stats__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TodosStats {
                    snapshot_at: snapshot_at__,
                    snapshot_id: snapshot_id__.unwrap_or_default(),
                    total_todos: total_todos__.unwrap_or_default(),
                    completed_todos: completed_todos__.unwrap_or_default(),
                    last_week_stats: last_week_stats__,
                    last_month_stats: last_month_stats__,
                    last_year_stats: last_year_stats__,
                    tag_stats: tag_stats__.unwrap_or_default(),
                    project_stats: project_stats__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.todos.v1.TodosStats", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("homelab.stats.todos.v1.UploadMetadata", len)?;
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
            let v = UploadTodosFileType::try_from(self.file_type)
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
                formatter.write_str("struct homelab.stats.todos.v1.UploadMetadata")
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
                            file_type__ = Some(map_.next_value::<UploadTodosFileType>()? as i32);
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
        deserializer.deserialize_struct("homelab.stats.todos.v1.UploadMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UploadTSodosError {
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
        let mut struct_ser = serializer.serialize_struct("homelab.stats.todos.v1.UploadTSodosError", len)?;
        if self.code != 0 {
            let v = UploadTodosErrorCode::try_from(self.code)
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
impl<'de> serde::Deserialize<'de> for UploadTSodosError {
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
            type Value = UploadTSodosError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.todos.v1.UploadTSodosError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UploadTSodosError, V::Error>
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
                            code__ = Some(map_.next_value::<UploadTodosErrorCode>()? as i32);
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
                Ok(UploadTSodosError {
                    code: code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    details: details__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.todos.v1.UploadTSodosError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UploadTodosErrorCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::ErrorTypeUnspecified => "UPLOAD_TODOS_ERROR_CODE_ERROR_TYPE_UNSPECIFIED",
            Self::InvalidSchema => "UPLOAD_TODOS_ERROR_CODE_INVALID_SCHEMA",
            Self::CorruptedFile => "UPLOAD_TODOS_ERROR_CODE_CORRUPTED_FILE",
            Self::ChecksumMismatch => "UPLOAD_TODOS_ERROR_CODE_CHECKSUM_MISMATCH",
            Self::DuplicateData => "UPLOAD_TODOS_ERROR_CODE_DUPLICATE_DATA",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for UploadTodosErrorCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UPLOAD_TODOS_ERROR_CODE_ERROR_TYPE_UNSPECIFIED",
            "UPLOAD_TODOS_ERROR_CODE_INVALID_SCHEMA",
            "UPLOAD_TODOS_ERROR_CODE_CORRUPTED_FILE",
            "UPLOAD_TODOS_ERROR_CODE_CHECKSUM_MISMATCH",
            "UPLOAD_TODOS_ERROR_CODE_DUPLICATE_DATA",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UploadTodosErrorCode;

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
                    "UPLOAD_TODOS_ERROR_CODE_ERROR_TYPE_UNSPECIFIED" => Ok(UploadTodosErrorCode::ErrorTypeUnspecified),
                    "UPLOAD_TODOS_ERROR_CODE_INVALID_SCHEMA" => Ok(UploadTodosErrorCode::InvalidSchema),
                    "UPLOAD_TODOS_ERROR_CODE_CORRUPTED_FILE" => Ok(UploadTodosErrorCode::CorruptedFile),
                    "UPLOAD_TODOS_ERROR_CODE_CHECKSUM_MISMATCH" => Ok(UploadTodosErrorCode::ChecksumMismatch),
                    "UPLOAD_TODOS_ERROR_CODE_DUPLICATE_DATA" => Ok(UploadTodosErrorCode::DuplicateData),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UploadTodosFileType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UPLOAD_TODOS_FILE_TYPE_UNSPECIFIED",
            Self::Sqlite => "UPLOAD_TODOS_FILE_TYPE_SQLITE",
            Self::Csv => "UPLOAD_TODOS_FILE_TYPE_CSV",
            Self::Json => "UPLOAD_TODOS_FILE_TYPE_JSON",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for UploadTodosFileType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UPLOAD_TODOS_FILE_TYPE_UNSPECIFIED",
            "UPLOAD_TODOS_FILE_TYPE_SQLITE",
            "UPLOAD_TODOS_FILE_TYPE_CSV",
            "UPLOAD_TODOS_FILE_TYPE_JSON",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UploadTodosFileType;

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
                    "UPLOAD_TODOS_FILE_TYPE_UNSPECIFIED" => Ok(UploadTodosFileType::Unspecified),
                    "UPLOAD_TODOS_FILE_TYPE_SQLITE" => Ok(UploadTodosFileType::Sqlite),
                    "UPLOAD_TODOS_FILE_TYPE_CSV" => Ok(UploadTodosFileType::Csv),
                    "UPLOAD_TODOS_FILE_TYPE_JSON" => Ok(UploadTodosFileType::Json),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UploadTodosRequest {
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
        let mut struct_ser = serializer.serialize_struct("homelab.stats.todos.v1.UploadTodosRequest", len)?;
        if let Some(v) = self.data.as_ref() {
            match v {
                upload_todos_request::Data::Metadata(v) => {
                    struct_ser.serialize_field("metadata", v)?;
                }
                upload_todos_request::Data::Chunk(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("chunk", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UploadTodosRequest {
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
            type Value = UploadTodosRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.todos.v1.UploadTodosRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UploadTodosRequest, V::Error>
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
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(upload_todos_request::Data::Metadata)
;
                        }
                        GeneratedField::Chunk => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunk"));
                            }
                            data__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| upload_todos_request::Data::Chunk(x.0));
                        }
                    }
                }
                Ok(UploadTodosRequest {
                    data: data__,
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.todos.v1.UploadTodosRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UploadTodosResponse {
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
        if self.todos_processed != 0 {
            len += 1;
        }
        if !self.warnings.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("homelab.stats.todos.v1.UploadTodosResponse", len)?;
        if self.status != 0 {
            let v = UploadTodosStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if self.bytes_received != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("bytesReceived", ToString::to_string(&self.bytes_received).as_str())?;
        }
        if self.todos_processed != 0 {
            struct_ser.serialize_field("todosProcessed", &self.todos_processed)?;
        }
        if !self.warnings.is_empty() {
            struct_ser.serialize_field("warnings", &self.warnings)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UploadTodosResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "bytes_received",
            "bytesReceived",
            "todos_processed",
            "todosProcessed",
            "warnings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            BytesReceived,
            TodosProcessed,
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
                            "todosProcessed" | "todos_processed" => Ok(GeneratedField::TodosProcessed),
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
            type Value = UploadTodosResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct homelab.stats.todos.v1.UploadTodosResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UploadTodosResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut bytes_received__ = None;
                let mut todos_processed__ = None;
                let mut warnings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<UploadTodosStatus>()? as i32);
                        }
                        GeneratedField::BytesReceived => {
                            if bytes_received__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bytesReceived"));
                            }
                            bytes_received__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TodosProcessed => {
                            if todos_processed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("todosProcessed"));
                            }
                            todos_processed__ = 
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
                Ok(UploadTodosResponse {
                    status: status__.unwrap_or_default(),
                    bytes_received: bytes_received__.unwrap_or_default(),
                    todos_processed: todos_processed__.unwrap_or_default(),
                    warnings: warnings__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("homelab.stats.todos.v1.UploadTodosResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UploadTodosStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UPLOAD_TODOS_STATUS_UNSPECIFIED",
            Self::Processing => "UPLOAD_TODOS_STATUS_PROCESSING",
            Self::Completed => "UPLOAD_TODOS_STATUS_COMPLETED",
            Self::Failed => "UPLOAD_TODOS_STATUS_FAILED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for UploadTodosStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UPLOAD_TODOS_STATUS_UNSPECIFIED",
            "UPLOAD_TODOS_STATUS_PROCESSING",
            "UPLOAD_TODOS_STATUS_COMPLETED",
            "UPLOAD_TODOS_STATUS_FAILED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UploadTodosStatus;

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
                    "UPLOAD_TODOS_STATUS_UNSPECIFIED" => Ok(UploadTodosStatus::Unspecified),
                    "UPLOAD_TODOS_STATUS_PROCESSING" => Ok(UploadTodosStatus::Processing),
                    "UPLOAD_TODOS_STATUS_COMPLETED" => Ok(UploadTodosStatus::Completed),
                    "UPLOAD_TODOS_STATUS_FAILED" => Ok(UploadTodosStatus::Failed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
