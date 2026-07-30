CC = gcc
CFLAGS = -Wall -Wextra -Iinclude -O2
LDFLAGS = -lm

SRC = src/main.c \
      src/law_envelope.c \
      src/admissibility_gate.c \
      src/immutable_core.c \
      src/reflex.c \
      src/ceal.c \
      src/governance.c \
      src/audit.c

OBJ = $(SRC:.c=.o)
TARGET = bin/bcind_core

all: $(TARGET)

$(TARGET): $(OBJ)
	@mkdir -p bin
	$(CC) $(CFLAGS) -o $@ $(OBJ) $(LDFLAGS)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -rf src/*.o bin/

.PHONY: all clean
