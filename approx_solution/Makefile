ifeq ($(OS),Windows_NT)
    EXE    := cs412_longestpath_approx.exe
    RM     := del /Q
    MKDIR  := if not exist
    SEP    := \\
else
    EXE    := cs412_longestpath_approx
    RM     := rm -f
    MKDIR  := mkdir -p
    SEP    := /
endif

CC       := cc
CFLAGS   := -O3 -Wall -Wextra -std=c99
LDFLAGS  :=
LDLIBS   := -lrt

SRC      := lp_approx.c graph.c
OBJ      := $(patsubst %.c,build/%.o,$(SRC))

BIN_DIR  := bin
BUILD_DIR := build
TARGET   := $(BIN_DIR)$(SEP)$(EXE)

all: $(TARGET)

$(TARGET): $(OBJ)
	$(MKDIR) $(BIN_DIR)
	$(CC) $(LDFLAGS) -o $@ $(OBJ) $(LDLIBS)

build/%.o: %.c
	$(MKDIR) $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	$(RM) $(BUILD_DIR)$(SEP)*.o
	$(RM) $(BIN_DIR)$(SEP)$(EXE)

rebuild: clean all

run: all
	./bin/$(EXE) < ./test_cases/example_0.txt

.PHONY: all clean rebuild run

