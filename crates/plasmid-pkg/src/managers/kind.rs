use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManagerKind {
    Brew,
    Apt,
    Winget,
}

impl ManagerKind {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Brew => "brew",
            Self::Apt => "apt",
            Self::Winget => "winget",
        }
    }
}

impl FromStr for ManagerKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "brew" => Ok(Self::Brew),
            "apt" => Ok(Self::Apt),
            "winget" => Ok(Self::Winget),
            _ => Err(()),
        }
    }
}
