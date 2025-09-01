use super::*;
use candid::Principal;
use std::cell::{Cell, RefCell};

use crate::models::Configuration;
use crate::repositories::ConfigurationRepositoryInterface;

#[derive(Default)]
struct MockRepo {
    stored: RefCell<Configuration>,
    save_calls: Cell<usize>,
}

impl MockRepo {
    fn with_initial(cfg: Configuration) -> Self {
        Self {
            stored: RefCell::new(cfg),
            save_calls: Cell::new(0),
        }
    }
    fn last_saved(&self) -> Configuration {
        self.stored.borrow().clone()
    }
    fn save_count(&self) -> usize {
        self.save_calls.get()
    }
}

impl ConfigurationRepositoryInterface for MockRepo {
    fn save(&self, cfg: Configuration) -> Configuration {
        self.save_calls.set(self.save_calls.get() + 1);
        self.stored.replace(cfg.clone());
        cfg
    }
    fn get(&self) -> Configuration {
        self.stored.borrow().clone()
    }
}

fn p(text: &str) -> Option<Principal> {
    Some(Principal::from_text(text).expect("valid principal"))
}

fn base_config() -> Configuration {
    Configuration::new(
        p("aaaaa-aa"),
        p("2vxsx-fae"),
        None,
        p("rwlgt-iiaaa-aaaaa-aaaaa-cai"),
        None,
        p("ryjl3-tyaaa-aaaaa-aaaba-cai"),
        None,
    )
}

#[test]
fn save_builds_config_and_delegates() {
    let repo = MockRepo::default();
    let service = ConfigurationService::new(repo);

    let out = service.save(
        p("aaaaa-aa"),
        p("2vxsx-fae"),
        None,
        p("rwlgt-iiaaa-aaaaa-aaaaa-cai"),
        None,
        p("ryjl3-tyaaa-aaaaa-aaaba-cai"),
        None,
    );

    // Returned value matches what we constructed
    assert_eq!(out.dao_agency_canister_id, p("aaaaa-aa"));
    assert_eq!(out.sogc_publication_canister_id, p("2vxsx-fae"));
    assert_eq!(out.dao_discovery_canister_id, None);
    assert_eq!(
        out.documents_storage_canister_id,
        p("rwlgt-iiaaa-aaaaa-aaaaa-cai")
    );
    assert_eq!(out.voting_canister_id, None);
    assert_eq!(
        out.network_call_canister_id,
        p("ryjl3-tyaaa-aaaaa-aaaba-cai")
    );
    assert_eq!(out.dao_platform_canister_id, None);

    // And the repo stored it exactly once
    let repo_ref = &service.configuration_repository;
    assert_eq!(repo_ref.save_count(), 1);
    let saved = repo_ref.last_saved();
    assert_eq!(
        saved.documents_storage_canister_id,
        p("rwlgt-iiaaa-aaaaa-aaaaa-cai")
    );
}

#[test]
fn get_passes_through_repo() {
    let initial = base_config();
    let repo = MockRepo::with_initial(initial.clone());
    let service = ConfigurationService::new(repo);

    let got = service.get();
    assert_eq!(got.dao_agency_canister_id, initial.dao_agency_canister_id);
    assert_eq!(
        got.network_call_canister_id,
        initial.network_call_canister_id
    );
}

#[test]
fn update_overwrites_only_some_fields() {
    let initial = base_config();
    let repo = MockRepo::with_initial(initial.clone());
    let service = ConfigurationService::new(repo);

    // Update only voting + dao_platform; leave others as None to preserve
    let updated = service.update(
        None,
        None,
        None,
        None,
        p("qaa6y-5yaaa-aaaaa-aaafa-cai"),
        None,
        p("qhbym-qaaaa-aaaaa-aaafq-cai"),
    );

    // Overwritten fields changed
    assert_eq!(updated.voting_canister_id, p("qaa6y-5yaaa-aaaaa-aaafa-cai"));
    assert_eq!(
        updated.dao_platform_canister_id,
        p("qhbym-qaaaa-aaaaa-aaafq-cai")
    );

    // Unspecified fields preserved from initial
    assert_eq!(
        updated.dao_agency_canister_id,
        initial.dao_agency_canister_id
    );
    assert_eq!(
        updated.sogc_publication_canister_id,
        initial.sogc_publication_canister_id
    );
    assert_eq!(
        updated.dao_discovery_canister_id,
        initial.dao_discovery_canister_id
    );
    assert_eq!(
        updated.documents_storage_canister_id,
        initial.documents_storage_canister_id
    );
    assert_eq!(
        updated.network_call_canister_id,
        initial.network_call_canister_id
    );

    // Repo saw one save during update
    let repo_ref = &service.configuration_repository;
    assert_eq!(repo_ref.save_count(), 1);
}
