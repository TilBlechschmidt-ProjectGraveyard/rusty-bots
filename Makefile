CARGO=cargo
CFLAGS=--release
LIB_PATH="user1/target/release"

ifeq ($(OS),Windows_NT)
    OS_detected := Windows
else
    OS_detected := $(shell uname -s)
endif

ifeq ($(OS_detected), Linux)
	LIBEXT=".so"
endif
ifeq ($(OS_detected), Darwin)
	LIBEXT=".dylib"
endif

all: server

run: server
	cd server && target/release/server

server: FORCE user1
	cd server && \
	$(CARGO) build $(CFLAGS)

user1: FORCE
	cd user1 && \
	$(CARGO) build $(CFLAGS)
	mkdir -p server/user && cp $(LIB_PATH)/libuser1$(LIBEXT) server/user/

client: FORCE
	cd client; \
	$(CARGO) build $(CFLAGS)


FORCE:
