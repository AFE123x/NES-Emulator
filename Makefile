# Compiler and flags
CC := clang
CFLAGS := -fsanitize=address,undefined -Og -g -Wall -Werror
TEST_FLAGS := -lcriterion
UNIT_TESTING := -DUNIT_TESTING

# Include path for Criterion
CRITERION_INCLUDE_PATH := /opt/homebrew/include/criterion  # Adjust as needed
CRITERION_LIB_PATH := /opt/homebrew/lib  # Adjust this path to where Criterion is installed

# Directories
SRC_DIR := src
OBJ_DIR := obj
BIN_DIR := bin

# Files
SRCS := $(wildcard $(SRC_DIR)/*.c)
OBJS := $(patsubst $(SRC_DIR)/%.c,$(OBJ_DIR)/%.o,$(SRCS))
TARGET := $(BIN_DIR)/main

# Default target
all: prod

# Production target (default behavior)
prod: CFLAGS := $(CFLAGS)
prod: LDFLAGS :=
prod: $(TARGET)

# Testing target (links with Criterion and defines UNIT_TESTING macro)
test: CFLAGS := $(CFLAGS) $(UNIT_TESTING) -I$(CRITERION_INCLUDE_PATH)
test: LDFLAGS := -L$(CRITERION_LIB_PATH) $(TEST_FLAGS)
test: $(TARGET)

# Link objects to create the executable
$(TARGET): $(OBJS) | $(BIN_DIR)
	$(CC) -o $@ $^ $(CFLAGS) $(LDFLAGS)

# Compile sources to objects
$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c | $(OBJ_DIR)
	$(CC) -o $@ -c $< $(CFLAGS)

# Create directories if they don't exist
$(BIN_DIR) $(OBJ_DIR):
	mkdir -p $@

# Clean up build files
clean:
	rm -rf $(OBJ_DIR) $(BIN_DIR)

# Phony targets
.PHONY: all clean prod test
