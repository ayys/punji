// Publishing a package
//
// - [ ] [compressing] compress the package into a tarball
// - [ ] [building] generate a webc file for the package
// - [ ] [pushing] upload the package to the server
// - [ ] [waiting-for-bindings] wait for the language bindings to be generated
// - [ ] [waiting-for-native-executables] waiting for the native executables to be generated
use std::path::Path;

use anyhow::Context;

struct Package;

impl Package {
    pub fn from_directory(_path: &Path) -> anyhow::Result<Self> {
        Ok(Self)
    }
}

fn compress_package(_package: &Package) -> anyhow::Result<()> {
    Ok(())
}
fn generate_webc(_package: &Package) -> anyhow::Result<()> {
    Ok(())
}
fn push_package_to_registry(_package: &Package) -> anyhow::Result<()> {
    Ok(())
}
fn wait_for_package_to_be_ready(_package: &Package) -> anyhow::Result<()> {
    Ok(())
}

pub fn publish_package_from_directory(path: &Path) -> anyhow::Result<()> {
    let package = Package::from_directory(path)?;
    compress_package(&package).with_context(|| "Error compressing package")?;
    generate_webc(&package).with_context(|| "Error generating webc for the package")?;
    push_package_to_registry(&package).with_context(|| "Error generating webc for the package")?;
    wait_for_package_to_be_ready(&package)
        .with_context(|| "Error generating webc for the package")?;
    Ok(())
}
