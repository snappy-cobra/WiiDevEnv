#---------------------------------------------------------------------------------
.SUFFIXES:
#---------------------------------------------------------------------------------

ifeq ($(strip $(DEVKITPPC)),)
$(error "Please set DEVKITPPC in your environment. export DEVKITPPC=<path to>devkitPPC)
endif

ifeq ($(strip $(DEVKITPRO)),)
$(error "Please set DEVKITPRO in your environment. export DEVKITPRO=<path to>devkitPro)
endif

include $(DEVKITPPC)/wii_rules

MACHDEP		+=	-DHW_RVL

INSTALL_INC	:=	$(DEVKITPRO)/portlibs/wii/include
INSTALL_LIB	:=	$(DEVKITPRO)/portlibs/wii/lib

NULLSTR :=
PWD     := $(subst $(NULLSTR) ,\ ,$(shell pwd))

INCLUDE	:= -I$(DEVKITPRO)/portlibs/ppc/include -I$(DEVKITPRO)/libogc/include
CFLAGS  := -O2 -Wall $(MACHDEP) $(INCLUDE)

LIB		:= tiny_physics
CFILES	:= $(wildcard *.c)
OFILES	:= $(CFILES:.c=.o)
DEPENDS	:= $(OFILES:.o=.d)
ARC		:= lib$(LIB).a
HDR		:= $(LIB).h

export DEPSDIR := $(CURDIR)

all : $(OFILES)
	$(AR) -r $(ARC) $(OFILES)

clean :
	rm -f $(OFILES) $(DEPENDS) $(ARC)

install :
	mkdir -p  $(INSTALL_LIB)  $(INSTALL_INC)
	cp -f $(ARC) $(INSTALL_LIB)/
	cp -f $(HDR) $(INSTALL_INC)/