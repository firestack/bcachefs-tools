PREFIX?=/usr/local
PKG_CONFIG?=pkg-config
INSTALL=install
RS_INCLUDE?=./rust-src/rbcachefs/include

LN=ln
CFLAGS+=-std=gnu89 -O2 -g -MMD -Wall -fPIC				\
	-Wno-pointer-sign					\
	-fno-strict-aliasing					\
	-fno-delete-null-pointer-checks				\
	-I. -Iinclude -Iraid					\
	-I$(RS_INCLUDE) \
	-D_FILE_OFFSET_BITS=64					\
	-D_GNU_SOURCE						\
	-D_LGPL_SOURCE						\
	-DRCU_MEMBARRIER					\
	-DZSTD_STATIC_LINKING_ONLY				\
	-DFUSE_USE_VERSION=32					\
	-DNO_BCACHEFS_CHARDEV					\
	-DNO_BCACHEFS_FS					\
	-DNO_BCACHEFS_SYSFS					\
	-DVERSION_STRING='"$(VERSION)"'				\
	$(EXTRA_CFLAGS)
LDFLAGS+=$(CFLAGS) $(EXTRA_LDFLAGS)

## Configure Tools
PYTEST_ARGS?=
PYTEST_CMD?=$(shell \
	command -v pytest-3 2>/dev/null \
	|| which pytest-3 2>/dev/null \
)
PYTEST:=$(PYTEST_CMD) $(PYTEST_ARGS)

RST2MAN_ARGS?=
RST2MAN_CMD?=$(shell \
	command -v rst2man \
	|| which rst2man 2>/dev/null \
	|| command -v rst2man.py \
	|| which rst2man.py 2>/dev/null \
)
RST2MAN:=$(RST2MAN_CMD) $(RST2MAN_ARGS)

CARGO_ARGS=
CARGO=cargo $(CARGO_ARGS)
# default is debug
# set to debug for default profile
CARGO_PROFILE_DIR?=release
# Unset for default profile
CARGO_PROFILE?=--release

CARGO_BUILD_ARGS=$(CARGO_PROFILE)
CARGO_BUILD=$(CARGO) build $(CARGO_BUILD_ARGS)
VERSION?=$(shell git describe --dirty=+ 2>/dev/null || echo v0.1-nogit)

include Makefile.compiler

CFLAGS+=$(call cc-disable-warning, unused-but-set-variable)
CFLAGS+=$(call cc-disable-warning, stringop-overflow)
CFLAGS+=$(call cc-disable-warning, zero-length-bounds)
CFLAGS+=$(call cc-disable-warning, missing-braces)
CFLAGS+=$(call cc-disable-warning, zero-length-array)
CFLAGS+=$(call cc-disable-warning, shift-overflow)
CFLAGS+=$(call cc-disable-warning, enum-conversion)

PKGCONFIG_LIBS="blkid uuid liburcu libsodium zlib liblz4 libzstd libudev"
ifdef BCACHEFS_FUSE
	PKGCONFIG_LIBS+="fuse3 >= 3.7"
	CFLAGS+=-DBCACHEFS_FUSE
endif

PKGCONFIG_CFLAGS:=$(shell $(PKG_CONFIG) --cflags $(PKGCONFIG_LIBS))
ifeq (,$(PKGCONFIG_CFLAGS))
    $(error pkg-config error, command: $(PKG_CONFIG) --cflags $(PKGCONFIG_LIBS))
endif
PKGCONFIG_LDLIBS:=$(shell $(PKG_CONFIG) --libs   $(PKGCONFIG_LIBS))
ifeq (,$(PKGCONFIG_LDLIBS))
    $(error pkg-config error, command: $(PKG_CONFIG) --libs $(PKGCONFIG_LIBS))
endif

CFLAGS+=$(PKGCONFIG_CFLAGS)
LDLIBS+=$(PKGCONFIG_LDLIBS)
LDLIBS+=-lm -lpthread -lrt -lkeyutils -laio -ldl
LDLIBS+=$(EXTRA_LDLIBS)

ifeq ($(PREFIX),/usr)
	ROOT_SBINDIR=/sbin
	INITRAMFS_DIR=$(PREFIX)/share/initramfs-tools
else
	ROOT_SBINDIR=$(PREFIX)/sbin
	INITRAMFS_DIR=/etc/initramfs-tools
endif

.PHONY: all
all: bcachefs bcachefs.5

.PHONY: rust
rust:  mount.bcachefs fsck.bcachefs sb_recover

.PHONY: tests
tests: tests/test_helper

.PHONY: check
check: tests bcachefs
ifneq (,$(PYTEST_CMD))
	$(PYTEST)
else
	@echo "WARNING: pytest not found or specified, tests could not be run."
endif

.PHONY: TAGS tags
TAGS:
	ctags -e -R .

tags:
	ctags -R .

DOCSRC := opts_macro.h bcachefs.5.rst.tmpl
DOCGENERATED := bcachefs.5 doc/bcachefs.5.rst
DOCDEPS := $(addprefix ./doc/,$(DOCSRC))
bcachefs.5: $(DOCDEPS)  libbcachefs/opts.h
ifneq (,$(RST2MAN_CMD))
	$(CC) doc/opts_macro.h -I libbcachefs -I include -E 2>/dev/null	\
		| doc/macro2rst.py
	$(RST2MAN) doc/bcachefs.5.rst bcachefs.5
else
	@echo "WARNING: no rst2man found! Man page not generated."
endif

SRCS=$(shell find . -type f -iname '*.c')
DEPS=$(SRCS:.c=.d)
-include $(DEPS)

OBJS=$(SRCS:.c=.o) librbcachefs.a
bcachefs: $(filter-out ./tests/%.o, $(OBJS))

mount.bcachefs: bcachefs
	$(LN) -f $+ $@

sb_recover: bcachefs
	$(LN) -f $+ $@

fsck.bcachefs: bcachefs
	$(LN) -f $+ $@

RUST_SRCS=$(shell find rust-src/ -type f -iname '*.rs')
MOUNT_SRCS=$(filter %mount, $(RUST_SRCS))

debug: CFLAGS+=-Werror -DCONFIG_BCACHEFS_DEBUG=y -DCONFIG_VALGRIND=y
debug: bcachefs

.PHONY: doc
doc:
	$(CARGO) doc --manifest-path rust-src/rbcachefs/Cargo.toml

librbcachefs.a: $(RUST_SRCS)
	$(CARGO_BUILD) --manifest-path rust-src/rbcachefs/Cargo.toml
	$(LN) -f rust-src/rbcachefs/target/$(CARGO_PROFILE_DIR)/librbcachefs.a $@

tests/test_helper: $(filter ./tests/%.o, $(OBJS))

# If the version string differs from the last build, update the last version
ifneq ($(VERSION),$(shell cat .version 2>/dev/null))
.PHONY: .version
endif
.version:
	echo '$(VERSION)' > $@

# Rebuild the 'version' command any time the version string changes
cmd_version.o : .version

.PHONY: install
install: INITRAMFS_HOOK=$(INITRAMFS_DIR)/hooks/bcachefs
install: INITRAMFS_SCRIPT=$(INITRAMFS_DIR)/scripts/local-premount/bcachefs
install: bcachefs
	$(INSTALL) -m0755 -D bcachefs      -t $(DESTDIR)$(ROOT_SBINDIR)
	$(LN) -s $(DESTDIR)$(ROOT_SBINDIR)/bcachefs $(DESTDIR)$(ROOT_SBINDIR)/mount.bcachefs
	$(LN) -s $(DESTDIR)$(ROOT_SBINDIR)/bcachefs $(DESTDIR)$(ROOT_SBINDIR)/fsck.bcachefs
	$(LN) -s $(DESTDIR)$(ROOT_SBINDIR)/bcachefs $(DESTDIR)$(ROOT_SBINDIR)/sb_recover
	$(INSTALL) -m0755    mkfs.bcachefs    $(DESTDIR)$(ROOT_SBINDIR)
	$(INSTALL) -m0644 -D bcachefs.8    -t $(DESTDIR)$(PREFIX)/share/man/man8/
	$(INSTALL) -m0755 -D initramfs/script $(DESTDIR)$(INITRAMFS_SCRIPT)
	$(INSTALL) -m0755 -D initramfs/hook   $(DESTDIR)$(INITRAMFS_HOOK)
	$(INSTALL) -m0755 -D mount.bcachefs.sh $(DESTDIR)$(ROOT_SBINDIR)
  
	sed -i '/^# Note: make install replaces/,$$d' $(DESTDIR)$(INITRAMFS_HOOK)
	echo "copy_exec $(ROOT_SBINDIR)/bcachefs /sbin/bcachefs" >> $(DESTDIR)$(INITRAMFS_HOOK)

.PHONY: clean
clean:
	$(RM) bcachefs mount.bcachefs libbcachefs_mount.a tests/test_helper .version $(OBJS) $(DEPS) $(DOCGENERATED)
	$(RM) -rf rust-src/*/target

.PHONY: deb
deb: all
	debuild -us -uc -nc -b -i -I

.PHONY: update-bcachefs-sources

WRKT:=$(shell mktemp -d)
update-bcachefs-sources:
## Create Worktree
	git -C $(LINUX_DIR) worktree add $(WRKT)

## Remove -tools libbcachefs
	git rm -rf --ignore-unmatch libbcachefs

## Create libbcachefs from kernel sources
	test -d libbcachefs || mkdir libbcachefs
	cp $(WRKT)/fs/bcachefs/*.[ch] libbcachefs/
	git add libbcachefs/*.[ch]
## More Bcachefs includes
	cp $(WRKT)/include/trace/events/bcachefs.h include/trace/events/
	git add include/trace/events/bcachefs.h
## include libraries from kernel
	cp $(WRKT)/include/linux/xxhash.h include/linux/
	git add include/linux/xxhash.h
	cp $(WRKT)/lib/xxhash.c linux/
	git add linux/xxhash.c
	cp $(WRKT)/kernel/locking/six.c linux/
	git add linux/six.c
	cp $(WRKT)/include/linux/six.h include/linux/
	git add include/linux/six.h
	cp $(WRKT)/include/linux/list_nulls.h include/linux/
	git add include/linux/list_nulls.h
	cp $(WRKT)/include/linux/poison.h include/linux/
	git add include/linux/poison.h
	
	cp $(WRKT)/scripts/Makefile.compiler ./
	git add Makefile.compiler
	
	$(RM) libbcachefs/*.mod.c
	
	git -C $(LINUX_DIR) rev-parse HEAD | tee .bcachefs_revision
	git add .bcachefs_revision
	
## Calculate Nix Hash which does not contain .git
## A Worktree is the easiest way to get a managed
## Checkout which can easily have .git destroyed
	rm $(WRKT)/.git
	nix hash path $(WRKT) > ./nix/bcachefs.rev.sha256
	git add ./nix/bcachefs.rev.sha256

## Cleanup worktree, repair damages
	git -C $(LINUX_DIR) worktree repair
	git -C $(LINUX_DIR) worktree remove $(WRKT)


.PHONY: update-commit-bcachefs-sources
update-commit-bcachefs-sources: update-bcachefs-sources
	git commit -m "Update bcachefs sources to $(shell git -C $(LINUX_DIR) show --oneline --no-patch)"
