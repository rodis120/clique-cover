
USERNAME ?= Rogal
APP_NAME ?= graph-generator

all: .image


.image: $(shell find .)
	docker build --tag ${USERNAME}/${APP_NAME} .
	touch ${@}
