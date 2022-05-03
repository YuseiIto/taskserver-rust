mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
mkfile_dir := $(dir $(mkfile_path))
DOCKER := docker
DOCKER_COMPOSE := docker compose
BASEDIR := $(mkfile_dir)testbench

TASKROOT := $(BASEDIR)/taskroot
TASK_DIR := $(TASKROOT)/.task
TASK_DIR_ON_CONTAINER := /root/.task
TASK_VOLUMES := -v $(TASKROOT):/root -v /etc/localtime:/etc/localtime:ro

NETWORK_INTERFACE = en0
IF_IP=$(shell ipconfig getifaddr $(NETWORK_INTERFACE))
TASK = $(DOCKER) run -i $(TASK_VOLUMES) --add-host=localhost:$(IF_IP) jamesnetherton/taskwarrior

TASKD_DIR = $(BASEDIR)/taskd
TASKD_CONTAINER = $(DOCKER) run -ti --rm --workdir /var/taskd/pki -v $(TASKD_DIR):/var/taskd connectical/taskd
TASKD = $(TASKD_CONTAINER) taskd


test_org_name:= public
test_user_name:= yuseiito

.PHONY: init-test-local
init-test-local:
	mkdir -p $(TASKROOT)
	echo "data.location=/root/.task" > $(TASKROOT)/.taskrc
	$(TASK) add 'First task'
	

.PHONY: init-test-remote
init-test-remote:
	$(DOCKER_COMPOSE) up -d
	sleep 5
	$(TASKD) add org $(test_org_name)
	echo $(test_org_name) > $(BASEDIR)/org_name
	$(TASKD) add user $(test_org_name) $(test_user_name) | grep 'New user key:' | sed 's/New user key: //' > $(BASEDIR)/user_key
	echo $(test_user_name) > $(BASEDIR)/user_name
	$(TASKD_CONTAINER) /var/taskd/pki/generate.client $(test_user_name)
	sleep 5

.PHONY: configure-remote
configure-remote:
	cp $(TASKD_DIR)/pki/ca.cert.pem	$(TASK_DIR)/
	cp $(TASKD_DIR)/pki/$(test_user_name).cert.pem	$(TASK_DIR)/
	cp $(TASKD_DIR)/pki/$(test_user_name).key.pem	$(TASK_DIR)/
	echo 'yes' | $(TASK) config taskd.certificate "$(TASK_DIR_ON_CONTAINER)/$(test_user_name).cert.pem"
	echo 'yes' | $(TASK) config taskd.key "$(TASK_DIR_ON_CONTAINER)/$(test_user_name).key.pem"
	echo 'yes' | $(TASK) config taskd.ca "$(TASK_DIR_ON_CONTAINER)/ca.cert.pem"
	echo 'yes' | $(TASK) config taskd.server localhost:53589
	echo 'yes' | $(TASK) config taskd.credentials '$(test_org_name)/$(test_user_name)/$(shell cat $(BASEDIR)/user_key)'
	sed -i -e 's/\\\//\//g' $(TASKROOT)/.taskrc
	sleep 3
	$(TASK) sync

.PHONY: stop-test-remote
stop-test-remote:
	$(DOCKER_COMPOSE) down

.PHONY: launch-testbench
launch-testbench: init-test-local init-test-remote configure-remote


.PHONY: clean-test-artifacts
clean-test-artifacts:
	rm -rf $(TASKD_DIR)
	rm -rf $(TASKROOT)
	rm $(BASEDIR)/org_name $(BASEDIR)/user_name $(BASEDIR)/user_key

.PHONY: halt-testbench
halt-testbench: stop-test-remote clean-test-artifacts
