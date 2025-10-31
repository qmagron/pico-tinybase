SRC := src
INC := src/include
BIN := bin

CC := arm-none-eabi-as
CC_FLAGS := -I$(INC)

OBJCOPY := arm-none-eabi-objcopy
OBJCOPY_FLAGS := -O binary --pad-to 0x0fc

CRC32 := ./src/crc32.py

INCS := $(wildcard $(INC)/*.S)
BINS := $(patsubst $(SRC)/%.S,$(BIN)/%.bin,$(wildcard $(SRC)/*.S))


all: $(BINS)


$(BIN)/%.bin: $(SRC)/%.S $(INCS)
	@echo ======== $@ ========
	@mkdir -p $(BIN)
	$(CC) $(CC_FLAGS) $< -o $(basename $@).o
	$(OBJCOPY) $(OBJCOPY_FLAGS) $(basename $@).o $@
	python3 $(CRC32) $@


clean:
	rm -rf $(BIN)


.PHONY: all clean
