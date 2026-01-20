use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CloudUsageOverviewInner {
    pub total: u64,
    pub used: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CloudUsageOverviewInnerFloat {
    pub total: f64,
    pub used: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CloudUsageOverview {
    pub vcpus: CloudUsageOverviewInner,
    pub ram: CloudUsageOverviewInner,
    pub gpus: CloudUsageOverviewInner,
    pub storage: CloudUsageOverviewInnerFloat,
    pub mwn_ips: CloudUsageOverviewInner,
    pub www_ips: CloudUsageOverviewInner,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CloudUsageFlavorSlot {
    pub name: String,
    pub free: u32,
    pub total: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CloudUsageAggregate {
    pub name: String,
    pub title: String,
    pub flavors: Vec<CloudUsageFlavorSlot>,
}

// TODO: how could we handle a table representation of this?
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CloudUsage {
    pub overview: CloudUsageOverview,
    pub lrz_flavor_slots: Vec<CloudUsageAggregate>,
    pub ach_flavor_slots: Vec<CloudUsageAggregate>,
    pub other_flavor_slots: Vec<CloudUsageAggregate>,
    pub datetime: String,
}
