# KAIRO Repository Cleanup Report

## Cleanup summary

This document summarizes the cleanup operations performed on the KAIRO repository as defined in the `kairo_repo_cleanup` job.

## Removed directories and files

The following files and directories have been removed:

*   `src/kairo_daemon.bak`
*   `src/kairo_daemon`
*   `src/kairo-daemon.bak`
*   `rust-core/src/log_recorder.rs`
*   `rust-core/tests`
*   `rust-core/examples`
*   `tools/clear-mini/clear-mini-daemon-job.json`
*   `vov/kairobot_ui`
*   `vov/old_ui`
*   `vov/legacy_ui`
*   `web/static_html`
*   `reports/old_*`
*   `target/debug`
*   `target/tmp`
*   `target/test`

## Post-cleanup tree (depth=3)

The repository structure after cleanup is as follows:

```
.
├── Cargo.lock
├── Cargo.toml
├── Dockerfile
├── Gemini.txt
├── LICENSE
├── Makefile
├── Nexus
│   ├── Directory.Build.props
│   ├── src
│   │   ├── Kairo.Core
│   │   ├── Kairo.Domain
│   │   ├── Kairo.Infrastructure
│   │   └── Kairo.Web
│   ├── Task.txt
│   └── tools
│       └── TestDataGenerator
├── ONBOARDING.md
├── PI-Vault
│   ├── configs
│   │   ├── app.config.yaml
│   │   ├── models.config.yaml
│   │   └── roles.config.yaml
│   ├── logs
│   ├── ProgressLogs
│   │   └── 2025-09-15_Log.md
│   ├── README.md
│   ├── RFCs
│   │   ├── RFC_2025-09-18_Self-Healing-Mesh.md
│   │   └── RFC_Template.md
│   ├── Roles
│   │   ├── AgentDeveloper.md
│   │   └── CoreTeam.md
│   ├── vault.config.yaml
│   ├── WAU_Refine
│   │   ├── WAU-2025-09-15.md
│   │   └── WAU-Template.md
│   └── Workflows
│       ├── AgentOnboarding.md
│       └── DailyStandup.md
├── Paramount
│   └── Workflow
│       └── README.md
├── QUICKSTART.md
├── README.md
├── README_old.md
├── REPO_OVERVIEW_Codex.txt
├── agent_config.json
├── agent_creation_summary.txt
├── agent_registry.json
├── agent_state
│   └── AGENT_LIST.md
├── all_usages.txt
├── archives
│   ├── KAIRO-Nexus_deprecated
│   │   ├── Kairo-Nexus
│   │   └── README.md
│   └── Nexus
│       ├── Directory.Build.props
│       ├── src
│       └── tools
├── check_daemon_response.py
├── cli_archives
│   ├── complete_20250630_172153.flag
│   ├── complete_20250630_172247.flag
│   ├── complete_20250630_173019.flag
│   ├── complete_20250630_232621.flag
│   ├── complete_20250630_235006.flag
│   ├── complete_20250630_235559.flag
│   ├── complete_20250630_235613.flag
│   ├── new_task_20250630_172153.json
│   ├── new_task_20250630_172247.json
│   ├── new_task_20250630_173019.json
│   ├── new_task_20250630_232621.json
│   ├── new_task_20250630_235006.json
│   ├── new_task_20250630_235559.json
│   ├── new_task_20250630_235613.json
│   ├── result_KAIRO-SEC-001.json
│   ├── result_KAIRO-SEC-002.json
│   └── result_KAIRO-SEC-003.json
├── clear-mini
│   ├── Cargo.toml
│   ├── README.md
│   └── src
│       ├── api.rs
│       ├── bin
│       ├── config.rs
│       ├── ise
│       ├── kairo_p.rs
│       ├── lib.rs
│       ├── time.rs
│       └── witness.rs
├── config
│   ├── daemon_pids.json
│   └── wau_thresholds.yml
├── configs
│   ├── roles
│   │   ├── agent_template.json
│   │   └── default_bot.json
│   └── workflows
│       ├── default.workflow.json
│       └── example.workflow.json
├── daemon_config.json
├── deploy.sh
├── diag_script.py
├── diagnose_windows.py
├── docs
│   ├── agent_configs
│   │   ├── agent_config_schema.json
│   │   └── example_agent_config.json
│   ├── Guidelines
│   │   ├── Agent-Onboarding.md
│   │   └── Code-Style.md
│   ├── kairo_daemon_api.md
│   ├── KAIROBOT_USAGE.md
│   ├── openai_api_compatibility_plan.md
│   ├── RFC
│   │   ├── RFC-template.md
│   │   └── sample-rfc.md
│   ├── RFC_AITCP_PACKET_v1.md
│   └── 端末識別方法.txt
├── fix_kairo_update.ps1
├── get_p_addresses.py
├── go-client
│   ├── client.go
│   ├── go.mod
│   └── README.md
├── go-p2p
│   ├── cmd
│   │   └── p2p
│   ├── go.mod
│   ├── go.sum
│   └── pkg
│       ├── discovery
│       ├── p2p
│       └── types
├── gpt_pid_candidates.ps1
├── handle_send_final_form.txt
├── identity
│   └── cli_identity.json
├── inbox_json
│   ├── cli-20250911004448-2216dfda.json
│   ├── cli-20250911171956-b0f83c8f.json
│   └── cli-20250911193433-fce1292c.json
├── logs
├── merge_codex_with_main.ps1
├── meshnet_cli_e2e_log.txt
├── patch_kairo.ps1
├── post_commit_validation_log.txt
├── registry.json
├── reports
│   ├── kairo_repo_cleanup.md
│   ├── kairo_repo_intake.md
│   ├── kairo_repo_patch.md
│   └── repo_tree_after_cleanup.md
├── router_fg.log
├── router_fg.pid
├── rust-core
│   ├── Cargo.toml
│   ├── benches
│   │   └── packet_parsing.rs
│   ├── src
│   │   ├── ai_tcp_packet_generated.rs
│   │   ├── bot
│   │   ├── ephemeral_session_generated.rs
│   │   ├── error.rs
│   │   ├── lib.rs
│   │   ├── main.rs
│   │   ├── mesh_trust_calculator.rs
│   │   ├── packet_parser.rs
│   │   └── packet_signer.rs
│   └── tests
│       └── packet_parser.rs
├── sample_envelopes.jsonl
├── samples
├── schema
│   ├── ai_tcp_packet.fbs
│   └── ephemeral_session.fbs
├── scripts
│   ├── check_duplicate_deps.py
│   ├── clear_agents.ps1
│   ├── generate_agent_list.py
│   ├── generate_kairo_pcap.py
│   ├── generate_mesh_config.py
│   ├── generate_test_pcaps.py
│   ├── renormalize.sh
│   ├── update_flatbuffers.py
│   ├── validate_logs.py
│   ├── verify_local_llm.ps1
│   └── write_work_results.sh
├── send_request.py
├── send_to_gpt.ps1
├── session_log_20250727.txt
├── src
│   ├── agent
│   │   ├── Cargo.toml
│   │   └── src
│   ├── bin
│   │   └── old_main.rs
│   ├── bot
│   │   ├── Cargo.toml
│   │   └── src
│   ├── errors.py
│   ├── governance
│   │   ├── Cargo.toml
│   │   └── src
│   ├── __init__.py
│   ├── kairo-daemon
│   │   ├── api
│   │   ├── Cargo.toml
│   │   ├── config.rs
│   │   ├── gpt_log_processor.rs
│   │   ├── gpt_responder.rs
│   │   ├── handle_send.rs
│   │   ├── handler.rs
│   │   ├── kairo_p
│   │   ├── kairo_p_listener.rs
│   │   ├── main.rs
│   │   ├── p_signature_validator.rs
│   │   ├── p_structure_filter.rs
│   │   └── task_queue.rs
│   ├── kairo-lib
│   │   ├── Cargo.toml
│   │   ├── comm.rs
│   │   ├── config.rs
│   │   ├── connection_manager.rs
│   │   ├── error.rs
│   │   ├── governance.rs
│   │   ├── lib.rs
│   │   ├── mesh_node.rs
│   │   ├── mesh_peer_discovery.rs
│   │   ├── mesh_scope_manager.rs
│   │   ├── packet.rs
│   │   ├── packet_parser.rs
│   │   ├── protocols
│   │   ├── py
│   │   ├── registry.rs
│   │   ├── resolvers
│   │   ├── seed_node_acl_manager.rs
│   │   ├── src
│   │   ├── tests
│   │   └── wau_config.rs
│   ├── kairo_lib
│   ├── kairof
│   │   ├── Cargo.toml
│   │   └── src
│   ├── log_recorder.py
│   ├── mesh-node
│   │   ├── Cargo.toml
│   │   └── src
│   └── server
│       ├── Cargo.toml
│       └── src
├── start_daemon.py
├── start_services.py
├── stop_services.py
├── target
│   ├── release
│   │   ├── .cargo-lock
│   │   ├── build
│   │   ├── deps
│   │   ├── examples
│   │   ├── kairo_agent_setup
│   │   ├── kairo_core_cli
│   │   ├── kairobot
│   │   ├── kairo-daemon
│   │   ├── libkairo_agent.rlib
│   │   ├── libkairobot.rlib
│   │   ├── libkairo_core.rlib
│   │   ├── libkairo_daemon.rlib
│   │   ├── libkairo_governance.rlib
│   │   ├── libkairo_lib.rlib
│   │   ├── libkairof.rlib
│   │   ├── libkairo_server.rlib
│   │   ├── libmesh_node.rlib
│   │   ├── libsetup_agent.rlib
│   │   ├── setup_agent
│   │   └── .fingerprint
│   └── .rustc_info.json
├── tash drop
├── test_multiple_tcp.ps1
├── test_tcp_8080.ps1
├── tests
│   ├── baseline_profile_test.rs
│   ├── behavior_anomaly_test.rs
│   ├── diagnosis_integration_test.rs
│   ├── mesh_auditor_test.rs
│   ├── packet_parser_test.rs
│   ├── packet_validator_test.rs
│   ├── setup_agent_smoke.rs
│   └── wau_config_test.rs
├── tests_py
│   ├── KAIRO_Autoloop.py
│   ├── test_errors.py
│   ├── test_generate_kairo_pcap.py
│   ├── test_generate_mesh_config.py
│   ├── test_generate_test_pcaps.py
│   ├── test_log_recorder.py
│   └── __pycache__
│       └── test_log_recorder.cpython-311.pyc
├── tmp_before.txt
├── tmp_chunk.txt
├── tmp_pd.txt
├── tools
│   └── clear-mini
├── usage_cli.md
├── users
│   ├── Agent1
│   │   ├── agent_config.json
│   │   └── private_key.pem
│   ├── Agent2
│   │   ├── agent_config.json
│   │   └── private_key.pem
│   └── CLI
│       ├── agent_config.json
│       └── private_key.pem
├── vov
│   ├── example_log.jsonl
│   ├── README.md
│   └── signing_key.txt
├── 作業レポート.md
└── クリップボード_10-04-2025_01.jpg
```
