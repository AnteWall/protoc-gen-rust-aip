use std::convert::TryFrom;

macro_rules! impl_name {
    ($t:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $t(String);

        impl $t {
            pub fn new(s: impl Into<String>) -> Self {
                Self(s.into())
            }
        }

        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl TryFrom<String> for $t {
            type Error = String;
            fn try_from(s: String) -> Result<Self, Self::Error> {
                if s.is_empty() {
                    Err("empty".to_string())
                } else {
                    Ok($t(s))
                }
            }
        }
    };
}

impl_name!(ProjectName);
impl_name!(TopicName);
impl_name!(BucketName);
impl_name!(UserName);
impl_name!(DocumentName);
impl_name!(DatabaseName);
impl_name!(InstanceName);
impl_name!(ObjectName);

pub use ProjectName;
pub use TopicName;
pub use BucketName;
pub use UserName;
pub use DocumentName;
pub use DatabaseName;
pub use InstanceName;
pub use ObjectName;
