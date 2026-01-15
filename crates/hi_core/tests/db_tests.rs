use hi_core::db::Store;
use hi_core::template::{Template, TemplateManifest};
use tempfile::NamedTempFile;

fn create_template(name: &str) -> Template {
    Template {
        manifest: TemplateManifest {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            authors: vec![],
            description: "".to_string(),
            repository: None,
            license: None,
            ignored: false,
        },
        targets: vec![],
        files: vec![],
        hooks: Default::default(),
    }
}

#[test]
fn test_db_persistence() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path();

    // 1. Create and populate
    {
        let mut db = Store::load(path).unwrap();
        db.add(create_template("theme_dark")).unwrap();
        db.add(create_template("icon_pack")).unwrap();
        db.save().unwrap();
    }

    // 2. Load and verify
    {
        let db = Store::load(path).unwrap();
        let list = db.list();
        assert_eq!(list.len(), 2);
        assert!(list.iter().any(|t| t.manifest.name == "theme_dark"));
        assert!(list.iter().any(|t| t.manifest.name == "icon_pack"));
    }
}

#[test]
fn test_add_remove() {
    let file = NamedTempFile::new().unwrap();
    let mut db = Store::load(file.path()).unwrap();

    // Add
    db.add(create_template("obsolete")).unwrap();
    assert_eq!(db.list().len(), 1);

    // Remove
    let removed = db.remove("obsolete");
    assert!(removed.is_some());
    assert_eq!(db.list().len(), 0);

    // Remove non-existent
    assert!(db.remove("ghost").is_none());
}

#[test]
fn test_update_template() {
    let file = NamedTempFile::new().unwrap();
    let mut db = Store::load(file.path()).unwrap();

    // Initial add v1
    let mut t1 = create_template("app");
    t1.manifest.version = "1.0.0".to_string();
    db.add(t1).unwrap();

    assert_eq!(db.list()[0].manifest.version, "1.0.0");

    // Update v2
    let mut t2 = create_template("app");
    t2.manifest.version = "2.0.0".to_string();
    db.add(t2).unwrap();

    assert_eq!(db.list()[0].manifest.version, "2.0.0");
}
