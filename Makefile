TARGET := hello
SOURCE := src/main.rs

ARCH := atmega32u4
PROGRAMMER := avr109
SERIAL_PORT := /dev/ttyACM0

TOPDIR := $(shell pwd)
BUILDDIR := $(TOPDIR)/target/$(ARCH)/release/

TARGET_ELF := $(BUILDDIR)/$(TARGET).elf
TARGET_HEX := $(BUILDDIR)/$(TARGET).hex

all: $(TARGET_ELF)

$(TARGET_ELF): $(SOURCE)
	XARGO_RUST_SRC=/ xargo build --target=$(ARCH) --release

$(TARGET_HEX): $(TARGET_ELF)
	avr-objcopy -j .text -j .data -O ihex $(TARGET_ELF) $(TARGET_HEX)

flash: $(TARGET_HEX)
	avrdude -p $(ARCH) -P $(SERIAL_PORT) -c $(PROGRAMMER) -v -U flash:w:$(TARGET_HEX)

clean:
	rm -rf $(TOPDIR)/target
