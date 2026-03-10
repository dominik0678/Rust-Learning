CC = gcc
CFLAGS = -Wall -Wextra -pedantic -std=c11
AR = ar
ARFLAGS = rcs

LIB = libtext_tools.a
OBJ = text_tools.o
MAIN_OBJ = main.o
DEMO = demo

.PHONY: all run clean

all: $(LIB) $(DEMO)

$(OBJ): text_tools.c text_tools.h
	$(CC) $(CFLAGS) -c text_tools.c -o $(OBJ)

$(LIB): $(OBJ)
	$(AR) $(ARFLAGS) $(LIB) $(OBJ)

$(MAIN_OBJ): main.c text_tools.h
	$(CC) $(CFLAGS) -c main.c -o $(MAIN_OBJ)

$(DEMO): $(MAIN_OBJ) $(LIB)
	$(CC) $(CFLAGS) $(MAIN_OBJ) -L. -ltext_tools -o $(DEMO)

run: $(DEMO)
	./$(DEMO)

clean:
	rm -f $(OBJ) $(MAIN_OBJ) $(LIB) $(DEMO)
