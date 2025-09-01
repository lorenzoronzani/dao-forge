use super::*;
use std::collections::HashMap;

#[test]
fn new_loads_default_templates() {
    let m = SogcPublicationTemplateManager::new();
    for k in [
        "dao_created",
        "dao_name_changed",
        "dao_member_added",
        "dao_member_removed",
        "dao_address_updated",
    ] {
        assert!(m.templates.contains_key(k));
    }
    assert_eq!(m.templates.len(), 5);
}

#[test]
fn render_replaces_placeholders_dao_created() {
    let m = SogcPublicationTemplateManager::new();
    let mut data = HashMap::new();
    data.insert("date".into(), "2024-01-02".into());
    data.insert("name".into(), "Acme".into());
    data.insert("address".into(), "Main St 1".into());
    data.insert("zip".into(), "8000".into());
    data.insert("town".into(), "Zürich".into());
    data.insert("uid".into(), "CHE-123.456.789".into());
    data.insert("ch_id".into(), "CH-020.3.012.345-6".into());
    data.insert("frc_id".into(), "42".into());
    data.insert("purpose".into(), "Governance".into());
    data.insert("board".into(), "Alice; Bob".into());
    data.insert("members".into(), "Carol; Dave".into());

    let out = m.render("dao_created", data).unwrap();
    for s in [
        "2024-01-02",
        "Acme",
        "Main St 1",
        "8000",
        "Zürich",
        "CHE-123.456.789",
        "CH-020.3.012.345-6",
        "42",
        "Governance",
        "Alice; Bob",
        "Carol; Dave",
    ] {
        assert!(out.contains(s));
    }
    assert!(!out.contains("{date}"));
    assert!(!out.contains("{name}"));
}

#[test]
fn render_unknown_template_err() {
    let m = SogcPublicationTemplateManager::new();
    let out = m.render("missing", HashMap::new());
    assert!(out.is_err());
    assert_eq!(out.unwrap_err(), "Template not found");
}

#[test]
fn render_leaves_unprovided_placeholders() {
    let m = SogcPublicationTemplateManager::new();
    let mut data = HashMap::new();
    data.insert("date".into(), "2024-01-02".into());
    data.insert("name".into(), "Acme".into());

    let out = m.render("dao_member_added", data).unwrap();
    assert!(out.contains("Acme"));
    assert!(out.contains("2024-01-02"));
    assert!(out.contains("{new_member}"));
    assert!(out.contains("{member_role}"));
}
