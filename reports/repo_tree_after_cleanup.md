.
├── Cargo.lock
├── Cargo.toml
├── Dockerfile
├── Gemini.txt
├── LICENSE
├── Makefile
├── Nexus
│   ├── Directory.Build.props
│   ├── Task.txt
│   ├── src
│   │   └── Kairo.ClipboardListener
│   └── tools
│       └── codex_clipboard_listener.ps1
├── ONBOARDING.md
├── PI-Vault
│   ├── ProgressLogs
│   │   └── 2025-07-09
│   ├── README.md
│   ├── RFCs
│   │   ├── 0001-example-rfc.md
│   │   └── 0002-another-rfc.md
│   ├── Roles
│   │   ├── analyst.yaml
│   │   └── resolver.yaml
│   ├── WAU_Refine
│   │   └── 2025-07-15_AI-TCP_スマホ構造化読解_異常解析.md
│   ├── Workflows
│   │   ├── Conflict_Resolution_Flow.md
│   │   └── Standard_Analysis_Workflow.md
│   ├── configs
│   │   └── roles
│   ├── logs
│   │   └── 2025-07-14-WAU-4layer-patch.md
│   └── vault.config.yaml
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
│   │   ├── Cargo.toml
│   │   ├── maps.json
│   │   ├── nexus_daemon.py
│   │   ├── src
│   │   ├── start_nexus.ps1
│   │   ├── stop_nexus.ps1
│   │   ├── tasks
│   │   └── utils.py
│   └── Nexus
│       ├── README.md
│       ├── config
│       ├── inbox
│       ├── inbox_json
│       ├── logs
│       ├── processed
│       ├── processed_json
│       ├── requirements.txt
│       ├── scripts
│       └── src
├── check_daemon_response.py
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
├── config
│   ├── daemon_pids.json
│   └── wau_thresholds.yml
├── configs
│   ├── roles
│   │   ├── README.md
│   │   ├── archivist.yaml
│   │   ├── auditor.yaml
│   │   ├── initiator.yaml
│   │   └── specialist.yaml
│   └── workflows
│       └── standard_analysis.yaml
├── daemon_config.json
├── deploy.sh
├── diag_script.py
├── diagnose_windows.py
├── docs
│   ├── Guidelines
│   │   └── GG01_operational_practices.md
│   ├── KAIROBOT_USAGE.md
│   ├── RFC
│   │   ├── AI-TCP_RFC_mesh_layered_architecture.md
│   │   ├── mesh_hierarchy.md
│   │   ├── mesh_ipv6.md
│   │   ├── mesh_node_discovery.md
│   │   └── peer_review_failure_modes.md
│   ├── RFC_AITCP_PACKET_v1.md
│   ├── agent_configs
│   │   └── 10.0.0.2.json
│   ├── kairo_daemon_api.md
│   ├── openai_api_compatibility_plan.md
│   └── 端末識別方法.txt
├── fix_kairo_update.ps1
├── get_p_addresses.py
├── go-client
│   ├── README.md
│   ├── client.go
│   └── go.mod
├── go-p2p
│   ├── cmd
│   │   ├── main.go
│   │   └── pcap_node
│   ├── go.mod
│   ├── go.sum
│   └── pkg
│       ├── generated
│       ├── handlers.go
│       ├── p2p_manager.go
│       ├── rust_bridge.go
│       └── serializer.go
├── gpt_pid_candidates.ps1
├── handle_send_final_form.txt
├── identity
│   └── cli_identity.json
├── inbox_json
│   ├── cli-20250911004448-2216dfda.json
│   ├── cli-20250911171956-b0f83c8f.json
│   └── cli-20250911193433-fce1292c.json
├── logs
│   ├── CoordinationNode_0001.log
│   ├── VoV_Trace.log
│   └── work_results.txt
├── merge_codex_with_main.ps1
├── meshnet_cli_e2e_log.txt
├── patch_kairo.ps1
├── post_commit_validation_log.txt
├── project_dump.zip
├── registry.json
├── reports
│   ├── kairo_repo_intake.md
│   ├── kairo_repo_patch.md
│   └── repo_tree_after_cleanup.md
├── router_fg.log
├── router_fg.pid
├── rust-core
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── benches
│   │   └── benchmark_flatbuffers.rs
│   ├── session_reuse.rs
│   └── src
│       ├── ai_tcp_packet_generated.rs
│       ├── baseline_profile_manager.rs
│       ├── bot
│       ├── compression.rs
│       ├── connection_manager.rs
│       ├── coordination
│       ├── ephemeral_session_generated.rs
│       ├── error.rs
│       ├── flatbuffers
│       ├── force_disconnect.rs
│       ├── fw_filter.rs
│       ├── keygen.rs
│       ├── lib.rs
│       ├── main.rs
│       ├── mesh.rs
│       ├── mesh_auditor.rs
│       ├── mesh_trust_calculator.rs
│       ├── packet_parser.rs
│       ├── packet_signer.rs
│       ├── packet_validator.rs
│       ├── rate_control.rs
│       ├── resolvers
│       ├── session.rs
│       └── signature.rs
├── sample_envelopes.jsonl
├── samples
│   ├── README.md
│   └── vov_log_sample.jsonl
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
│   ├── __init__.py
│   ├── agent
│   │   ├── Cargo.toml
│   │   ├── agent_config.json
│   │   ├── check_registry.rs
│   │   ├── forged_sender.rs
│   │   ├── mesh_udp_sender.rs
│   │   ├── receive_signed.rs
│   │   ├── send_message.rs
│   │   ├── setup_agent.rs
│   │   ├── signed_sender.rs
│   │   └── validate_config.rs
│   ├── bin
│   │   └── setup_agent.rs
│   ├── bot
│   │   ├── Cargo.toml
│   │   ├── api
│   │   ├── main.rs
│   │   └── ui
│   ├── errors.py
│   ├── governance
│   │   ├── Cargo.toml
│   │   ├── propose_override.rs
│   │   └── sign_override.rs
│   ├── kairo-daemon
│   │   ├── Cargo.toml
│   │   ├── api
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
│   │   ├── generate_pcap.rs
│   │   └── lib.rs
│   ├── log_recorder.py
│   ├── mesh-node
│   │   ├── Cargo.toml
│   │   ├── main.rs
│   │   ├── mesh_node.rs
│   │   ├── seed_node.rs
│   │   └── src
│   └── server
│       ├── Cargo.toml
│       ├── seed_node.rs
│       ├── seed_node_mock.rs
│       └── src
├── start_daemon.py
├── start_services.py
├── stop_services.py
├── target
│   ├── CACHEDIR.TAG
│   └── release
│       ├── build
│       ├── check_registry
│       ├── check_registry.d
│       ├── clear-mini-daemon
│       ├── clear-mini-daemon.d
│       ├── deps
│       ├── examples
│       ├── forged_sender
│       ├── forged_sender.d
│       ├── incremental
│       ├── kairo-daemon
│       ├── kairo-daemon.d
│       ├── kairo_agent_setup
│       ├── kairo_agent_setup.d
│       ├── kairo_core_cli
│       ├── kairo_core_cli.d
│       ├── kairo_server
│       ├── kairo_server.d
│       ├── kairobot
│       ├── kairobot.d
│       ├── libclear_mini.d
│       ├── libclear_mini.rlib
│       ├── libkairo_core.d
│       ├── libkairo_core.rlib
│       ├── libkairo_lib.d
│       ├── libkairo_lib.rlib
│       ├── libkairof.d
│       ├── libkairof.rlib
│       ├── mesh_node
│       ├── mesh_node.d
│       ├── mesh_udp_sender
│       ├── mesh_udp_sender.d
│       ├── propose_override
│       ├── propose_override.d
│       ├── receive_signed
│       ├── receive_signed.d
│       ├── seed_node
│       ├── seed_node.d
│       ├── seed_node_mock
│       ├── seed_node_mock.d
│       ├── send_message
│       ├── send_message.d
│       ├── setup_agent
│       ├── setup_agent.d
│       ├── signed_sender
│       ├── signed_sender.d
│       ├── validate_config
│       └── validate_config.d
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
│   ├── __pycache__
│   │   └── test_generate_kairo_pcap.cpython-310.pyc
│   ├── test_errors.py
│   ├── test_generate_kairo_pcap.py
│   ├── test_generate_mesh_config.py
│   ├── test_generate_test_pcaps.py
│   └── test_log_recorder.py
├── tmp_before.txt
├── tmp_chunk.txt
├── tmp_pd.txt
├── tools
│   └── clear-mini
├── usage_cli.md
├── users
│   ├── Agent1
│   │   └── agent_config.json
│   ├── Agent2
│   │   └── agent_config.json
│   └── CLI
│       ├── agent_config.json
│       └── agent_configs
├── vov
│   ├── README.md
│   ├── example_log.jsonl
│   └── signing_key.txt
├── クリップボード_10-04-2025_01.jpg
├── 作業レポート.md
└── 作業ログ.txt

101 directories, 299 files
