TARGET = test

SRCS = main.c adj_list.c clique.c base64.c

CFLAGS = -Wall -static

all: $(TARGET)

$(TARGET): $(SRCS)
	$(CC) $(CFLAGS) -o $@ $^

dbg: CFLAGS += -g -DDEBUG
dbg: $(TARGET)

clean:
	rm -f $(TARGET)

.PHONY: all clean dbg
