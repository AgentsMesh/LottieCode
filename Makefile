# LottieCode Makefile —— 常用任务的薄壳，本质都委托给 Bazel。
#
# 主要 target：
#   build      —— bazel build 优化版 CLI
#   test       —— bazel test 全量
#   install    —— 把 CLI 安装到 $(BINDIR)/$(BIN_NAME)
#   uninstall  —— 卸载
#   clean      —— bazel clean
#
# 自定义安装路径：`make install PREFIX=/usr/local`

PREFIX    ?= $(HOME)/.local
BINDIR    := $(PREFIX)/bin
BIN_NAME  := lc

BAZEL     ?= bazel
TARGET    := //crates/lottiecode-cli:lottiecode-cli
BAZEL_BIN := bazel-bin/crates/lottiecode-cli/lottiecode-cli

.PHONY: help build test install uninstall clean

help:
	@echo "LottieCode —— 常用任务："
	@echo "  make build      bazel build -c opt CLI"
	@echo "  make test       bazel test //..."
	@echo "  make install    安装 lc 到 $(BINDIR)/$(BIN_NAME)"
	@echo "  make uninstall  卸载 $(BINDIR)/$(BIN_NAME)"
	@echo "  make clean      bazel clean"
	@echo ""
	@echo "覆盖默认值："
	@echo "  make install PREFIX=/usr/local   # 安装到 /usr/local/bin"
	@echo "  make BAZEL=bazelisk build        # 用 bazelisk 替代 bazel"

build:
	$(BAZEL) build -c opt $(TARGET)

test:
	$(BAZEL) test //... --test_output=errors

install: build
	@mkdir -p $(BINDIR)
	@cp -f $(BAZEL_BIN) $(BINDIR)/$(BIN_NAME)
	@chmod +x $(BINDIR)/$(BIN_NAME)
	@echo "✔ 已安装 $(BINDIR)/$(BIN_NAME)"
	@case ":$$PATH:" in \
		*":$(BINDIR):"*) ;; \
		*) echo "⚠ $(BINDIR) 不在 PATH 中。添加："; \
		   echo "    echo 'export PATH=\"$(BINDIR):\$$PATH\"' >> ~/.zshrc" ;; \
	esac

uninstall:
	@rm -f $(BINDIR)/$(BIN_NAME)
	@echo "✔ 已卸载 $(BINDIR)/$(BIN_NAME)"

clean:
	$(BAZEL) clean
