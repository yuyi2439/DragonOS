
ifeq ($(ARCH), )
# ！！！！在这里设置ARCH，可选 x86_64 和 riscv64
# !!!!!!!如果不同时调整这里以及vscode的settings.json，那么自动补全和检查将会失效
export ARCH?=x86_64
endif

ifeq ($(EMULATOR), )
export EMULATOR=__NO_EMULATION__
endif


export DEBUG=DEBUG

export CFLAGS_DEFINE_ARCH="__$(ARCH)__"

export GLOBAL_CFLAGS := -fno-builtin -fno-stack-protector -D $(CFLAGS_DEFINE_ARCH) -D $(EMULATOR) -O1

ifeq ($(ARCH), x86_64)
GLOBAL_CFLAGS += -mcmodel=large -m64
else ifeq ($(ARCH), riscv64)
GLOBAL_CFLAGS += -mcmodel=medany -march=rv64imac -mabi=lp64
endif

ifeq ($(DEBUG), DEBUG)
GLOBAL_CFLAGS += -g 
endif