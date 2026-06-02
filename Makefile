CC := cc
C_FLAGS := -g -Wall -Wextra -Werror -fPIC
SOURCE_DIR := src
SOURCE_FILES := $(wildcard $(SOURCE_DIR)/*.c) $(wildcard $(SOURCE_DIR)/*/*.c)
BUILD_DIR := build
DYNAMIC_LINKS := -lOpenGL -lglfw


build: clean
	mkdir build
	$(CC) -shared $(C_FLAGS) $(DYNAMIC_LINKS) $(SOURCE_FILES) -o $(BUILD_DIR)/libpixl.so

runtestexample: build
	$(CC) -g -Wall -Wextra -Werror -o $(BUILD_DIR)/pixltestexample src/testexample.c -L$(BUILD_DIR) -lpixl $(DYNAMIC_LINKS) 
	LD_LIBRARY_PATH=build ./build/pixltestexample

clean:
	rm -rf build
