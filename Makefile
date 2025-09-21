PROFILE ?= dev

ifneq (,$(filter prod,$(MAKECMDGOALS)))
  PROFILE := prod
  MAKECMDGOALS := $(filter-out prod,$(MAKECMDGOALS))
endif

ifneq (,$(filter dev,$(MAKECMDGOALS)))
  PROFILE := dev
  MAKECMDGOALS := $(filter-out dev,$(MAKECMDGOALS))
endif

ifeq ($(PROFILE),dev)
  COMPOSE := docker compose -f compose.dev.yml
else ifeq ($(PROFILE),prod)
  COMPOSE := docker compose -f compose.prod.yml -f compose.prod.local.yml
else
  $(error Unknown PROFILE=$(PROFILE))
endif

.PHONY: up build down %
up:
	$(COMPOSE) up $(filter-out $@,$(MAKECMDGOALS))
build:
	$(COMPOSE) build $(filter-out $@,$(MAKECMDGOALS))
down:
	$(COMPOSE) down $(filter-out $@,$(MAKECMDGOALS))

.PHONY: dev prod
dev: ;
prod: ;



-include Makefile.local