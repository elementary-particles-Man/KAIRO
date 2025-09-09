use kairo_lib::wau_config;

#[test]
fn load_wau_thresholds() {
    let cfg = wau_config::WauThresholds::load_from("config/wau_thresholds.yml").unwrap();
    assert!(cfg.world > cfg.personal);
}
