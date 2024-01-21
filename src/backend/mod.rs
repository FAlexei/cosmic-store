use appstream::Collection;
use cosmic::widget;
use std::{collections::HashMap, error::Error, sync::Arc};

use crate::AppstreamCache;

#[cfg(feature = "flatpak")]
mod flatpak;

#[cfg(feature = "packagekit")]
mod packagekit;

#[derive(Clone, Debug)]
pub struct Package {
    pub id: String,
    pub icon: widget::icon::Named,
    pub name: String,
    pub version: String,
    pub extra: HashMap<String, String>,
}

pub trait Backend {
    fn installed(&self) -> Result<Vec<Package>, Box<dyn Error>>;
    fn appstream(&self, package: &Package) -> Result<Collection, Box<dyn Error>>;
}

pub fn backends(
    appstream_cache: &Arc<AppstreamCache>,
    locale: &str,
) -> HashMap<&'static str, Arc<dyn Backend>> {
    let mut backends = HashMap::<&'static str, Arc<dyn Backend>>::new();

    #[cfg(feature = "flatpak")]
    {
        match flatpak::Flatpak::new() {
            Ok(backend) => {
                backends.insert("flatpak", Arc::new(backend));
            }
            Err(err) => {
                log::error!("failed to load flatpak backend: {}", err);
            }
        }
    }

    #[cfg(feature = "packagekit")]
    {
        match packagekit::Packagekit::new(appstream_cache, locale) {
            Ok(backend) => {
                backends.insert("packagekit", Arc::new(backend));
            }
            Err(err) => {
                log::error!("failed to load packagekit backend: {}", err);
            }
        }
    }

    backends
}
