use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct BrewVersions {
    pub stable: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct BrewFormulae {
    pub name: String,
    pub versions: BrewVersions,
    pub installed: Vec<BrewInstalled>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct BrewInstalled {
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct BrewInfo {
    pub formulae: Vec<BrewFormulae>,
}
