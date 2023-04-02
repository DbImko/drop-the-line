prog :=drop-the-line

DESTDIR = ${HOME}/bin

debug ?=

ifdef debug
  release :=
  target :=debug
  extension :=debug
else
  release :=--release
  target :=release
  extension :=
endif

build:
	cargo build $(release)

install:
	mkdir -p ${DESTDIR}
	cp target/$(target)/$(prog) ${DESTDIR}/dtl

all: build install
 
help:
	@echo "usage: make $(prog) [debug=1]"