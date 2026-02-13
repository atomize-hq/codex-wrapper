use claude_code::{
    PluginDisableRequest, PluginEnableRequest, PluginListRequest, PluginMarketplaceListRequest,
    PluginMarketplaceRequest, PluginRequest, PluginUpdateRequest, PluginValidateRequest,
};

#[test]
fn plugin_root_argv() {
    let argv = PluginRequest::new().into_command().argv();
    assert_eq!(argv, ["plugin"]);

    let argv = PluginMarketplaceRequest::new().into_command().argv();
    assert_eq!(argv, ["plugin", "marketplace"]);
}

#[test]
fn plugin_enable_orders_scope_before_plugin_name() {
    let argv = PluginEnableRequest::new("my-plugin")
        .scope("project")
        .into_command()
        .argv();
    assert_eq!(
        argv,
        ["plugin", "enable", "--scope", "project", "my-plugin"]
    );
}

#[test]
fn plugin_disable_orders_flags_before_positionals() {
    let argv = PluginDisableRequest::new()
        .all(true)
        .scope("user")
        .into_command()
        .argv();
    assert_eq!(argv, ["plugin", "disable", "--all", "--scope", "user"]);
}

#[test]
fn plugin_list_orders_flags_deterministically() {
    let argv = PluginListRequest::new()
        .available(true)
        .json(true)
        .into_command()
        .argv();
    assert_eq!(argv, ["plugin", "list", "--available", "--json"]);
}

#[test]
fn plugin_update_includes_scope_before_plugin_name() {
    let argv = PluginUpdateRequest::new("my-plugin")
        .scope("project")
        .into_command()
        .argv();
    assert_eq!(
        argv,
        ["plugin", "update", "--scope", "project", "my-plugin"]
    );
}

#[test]
fn plugin_validate_includes_path_positional() {
    let argv = PluginValidateRequest::new("path/to/plugin")
        .into_command()
        .argv();
    assert_eq!(argv, ["plugin", "validate", "path/to/plugin"]);
}

#[test]
fn plugin_marketplace_list_includes_json_flag() {
    let argv = PluginMarketplaceListRequest::new()
        .json(true)
        .into_command()
        .argv();
    assert_eq!(argv, ["plugin", "marketplace", "list", "--json"]);
}
