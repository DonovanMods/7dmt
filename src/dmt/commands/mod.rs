use crate::cli::RequestedVersion;

pub mod bump;
pub mod convert;
pub mod init;
pub mod validate;

pub fn requested_version_to_modinfo_version(requested_version: &Option<RequestedVersion>) -> modinfo::ModinfoVersion {
    match requested_version {
        Some(ver) => match ver {
            _ if ver.v1 => modinfo::ModinfoVersion::V1,
            _ if ver.v2 => modinfo::ModinfoVersion::V2,
            _ => modinfo::ModinfoVersion::V2,
        },
        None => modinfo::ModinfoVersion::V2,
    }
}
