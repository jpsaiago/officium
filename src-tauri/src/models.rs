use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationStatus {
    Saved,
    Applied,
    Interviewing,
    Offer,
    Rejected,
    Withdrawn,
}

impl ApplicationStatus {
    fn as_str(&self) -> &'static str {
        match self {
            ApplicationStatus::Saved => "saved",
            ApplicationStatus::Applied => "applied",
            ApplicationStatus::Interviewing => "interviewing",
            ApplicationStatus::Offer => "offer",
            ApplicationStatus::Rejected => "rejected",
            ApplicationStatus::Withdrawn => "withdrawn",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "saved" => Some(Self::Saved),
            "applied" => Some(Self::Applied),
            "interviewing" => Some(Self::Interviewing),
            "offer" => Some(Self::Offer),
            "rejected" => Some(Self::Rejected),
            "withdrawn" => Some(Self::Withdrawn),
            _ => None,
        }
    }
}

// Store the enum as its lowercase name rather than an integer so the raw
// .sqlite3 file stays human-readable if inspected directly.
impl ToSql for ApplicationStatus {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.as_str()))
    }
}

impl FromSql for ApplicationStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let text = value.as_str()?;
        Self::from_str(text).ok_or(FromSqlError::InvalidType)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkMode {
    Remote,
    Hybrid,
    Onsite,
}

impl WorkMode {
    fn as_str(&self) -> &'static str {
        match self {
            WorkMode::Remote => "remote",
            WorkMode::Hybrid => "hybrid",
            WorkMode::Onsite => "onsite",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "remote" => Some(Self::Remote),
            "hybrid" => Some(Self::Hybrid),
            "onsite" => Some(Self::Onsite),
            _ => None,
        }
    }
}

impl ToSql for WorkMode {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.as_str()))
    }
}

impl FromSql for WorkMode {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let text = value.as_str()?;
        Self::from_str(text).ok_or(FromSqlError::InvalidType)
    }
}

// Brazil-specific hiring arrangement: PJ (pessoa jurídica, invoiced as a
// contractor) vs CLT (salaried employee under labor law).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractType {
    Pj,
    Clt,
}

impl ContractType {
    fn as_str(&self) -> &'static str {
        match self {
            ContractType::Pj => "pj",
            ContractType::Clt => "clt",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "pj" => Some(Self::Pj),
            "clt" => Some(Self::Clt),
            _ => None,
        }
    }
}

impl ToSql for ContractType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.as_str()))
    }
}

impl FromSql for ContractType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let text = value.as_str()?;
        Self::from_str(text).ok_or(FromSqlError::InvalidType)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Application {
    pub id: i64,
    pub company: String,
    pub role: String,
    pub status: ApplicationStatus,
    pub job_url: Option<String>,
    pub location: Option<String>,
    pub work_mode: Option<WorkMode>,
    pub contract_type: Option<ContractType>,
    pub salary: Option<String>,
    pub applied_on: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewApplication {
    pub company: String,
    pub role: String,
    #[serde(default)]
    pub status: Option<ApplicationStatus>,
    #[serde(default)]
    pub job_url: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub work_mode: Option<WorkMode>,
    #[serde(default)]
    pub contract_type: Option<ContractType>,
    #[serde(default)]
    pub salary: Option<String>,
    #[serde(default)]
    pub applied_on: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
}

/// All fields optional: only the ones present in the payload get written,
/// via `COALESCE(?, column)` in the UPDATE statement.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateApplication {
    #[serde(default)]
    pub company: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub status: Option<ApplicationStatus>,
    #[serde(default)]
    pub job_url: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub work_mode: Option<WorkMode>,
    #[serde(default)]
    pub contract_type: Option<ContractType>,
    #[serde(default)]
    pub salary: Option<String>,
    #[serde(default)]
    pub applied_on: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
}
