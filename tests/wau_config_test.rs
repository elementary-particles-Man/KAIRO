#[test]
fn load_wau_thresholds(){ let cfg=crate::wau_config::WauThresholds::load_from("config/wau_thresholds.yml").unwrap(); assert!(cfg.world>cfg.personal); }
