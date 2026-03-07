use std::{collections::HashMap, sync::Arc};

use crate::{
    managers::{
        apt::AptManager, brew::BrewManager, kind::ManagerKind, manager::PackageManager,
        winget::WingetManager,
    },
    runner::commandrunner::CommandRunner,
};

pub struct ManagerRegistry {
    managers: HashMap<ManagerKind, Box<dyn PackageManager>>,
}

impl ManagerRegistry {
    pub fn new(runner: Arc<dyn CommandRunner>) -> Self {
        let mut managers: HashMap<ManagerKind, Box<dyn PackageManager>> = HashMap::new();

        managers.insert(
            ManagerKind::Brew,
            Box::new(BrewManager::new(runner.clone())),
        );
        managers.insert(ManagerKind::Apt, Box::new(AptManager::new(runner.clone())));
        managers.insert(ManagerKind::Winget, Box::new(WingetManager::new(runner)));

        Self { managers }
    }

    pub fn resolve(&self, requested: Option<&str>) -> Option<&dyn PackageManager> {
        if let Some(req) = requested
            && let Ok(kind) = req.parse::<ManagerKind>()
        {
            return self.managers.get(&kind).map(AsRef::as_ref);
        }

        #[cfg(target_os = "macos")]
        {
            if let Some(m) = self.managers.get(&ManagerKind::Brew)
                && m.available().is_ok()
            {
                return Some(m.as_ref());
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Some(m) = self.managers.get(&ManagerKind::Apt)
                && m.available().is_ok()
            {
                return Some(m.as_ref());
            }
            if let Some(m) = self.managers.get(&ManagerKind::Brew)
                && m.available().is_ok()
            {
                return Some(m.as_ref());
            }
        }

        #[cfg(target_os = "windows")]
        {
            if let Some(m) = self.managers.get(&ManagerKind::Winget)
                && m.available().is_ok()
            {
                return Some(m.as_ref());
            }
        }

        None
    }
}
