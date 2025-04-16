PROTO_PATH := ../sepia2-grpc/proto
PROTO_FILE := $(PROTO_PATH)/api.proto
BUILD_DIR := src/sepia2_client

proto: $(PROTO_FILE) $(BUILD_DIR)/
	python -m grpc_tools.protoc -I$(PROTO_PATH) --python_out=$(BUILD_DIR) --grpc_python_out=$(BUILD_DIR) $(PROTO_FILE)
