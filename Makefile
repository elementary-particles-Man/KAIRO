FLATC ?= flatc

.PHONY: generate_schema
generate_schema:
$(FLATC) --rust -o rust-core/src/ schema/ai_tcp_packet.fbs
$(FLATC) --go -o go-p2p/pkg/generated/AITCP/ schema/ai_tcp_packet.fbs

.PHONY: check_schema
check_schema: generate_schema
@status=`git status --porcelain`; \
if [ -n "$$status" ]; then \
  echo "FlatBuffers generated files are not up-to-date."; \
  git diff; \
  exit 1; \
else \
  echo "Schema is up-to-date."; \
fi
