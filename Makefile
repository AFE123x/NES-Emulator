#
# 'make'        build executable file 'main'
# 'make clean'  removes all .o and executable files
#

# define the C++ compiler to use
CXX = g++

# define any compile-time flags
CXXFLAGS := -std=c++17 -Wall -Wextra -g -fsanitize=address,undefined -Og
LDFLAGS  := -lSDL2 -lSDL2_ttf

# define output directory
OUTPUT   := output

# define source directory
SRC      := src

# define include directory
INCLUDE  := include

# define lib directory
LIB      := lib

ifeq ($(OS),Windows_NT)
MAIN        := NES.exe
SOURCEDIRS  := $(SRC)
INCLUDEDIRS := $(INCLUDE)
LIBDIRS     := $(LIB)
FIXPATH     = $(subst /,\,$1)
RM          := del /q /f
MD          := mkdir
else
MAIN        := NES
SOURCEDIRS  := $(shell find $(SRC) -type d)
INCLUDEDIRS := $(shell find $(INCLUDE) -type d)
LIBDIRS     := $(shell find $(LIB) -type d)
FIXPATH     = $1
RM          := rm -f
MD          := mkdir -p
endif

# define any directories containing header files other than /usr/include
INCLUDES    := $(patsubst %,-I%, $(INCLUDEDIRS:%/=%))

# define the library paths
LIBS        := $(patsubst %,-L%, $(LIBDIRS:%/=%))

# define the C++ source files
SOURCES     := $(wildcard $(patsubst %,%/*.cpp, $(SOURCEDIRS)))

# define the C++ object files
OBJECTS     := $(SOURCES:.cpp=.o)

# define the dependency output files
DEPS        := $(OBJECTS:.o=.d)

#
# The following part of the Makefile is generic; it can be used to
# build any executable just by changing the definitions above and by
# deleting dependencies appended to the file from 'make depend'
#

OUTPUTMAIN  := $(call FIXPATH,$(OUTPUT)/$(MAIN))

all: $(OUTPUT) $(MAIN)
	@echo Executing 'all' complete!

$(OUTPUT):
	$(MD) $(OUTPUT)

$(MAIN): $(OBJECTS)
	$(CXX) $(CXXFLAGS) $(INCLUDES) -o $(OUTPUTMAIN) $(OBJECTS) $(LDFLAGS) $(LIBS)

# include all .d files
-include $(DEPS)

# this is a suffix replacement rule for building .o's and .d's from .cpp's
# it uses automatic variables $<: the name of the prerequisite of
# the rule(a .cpp file) and $@: the name of the target of the rule (a .o file)
# -MMD generates dependency output files same name as the .o file
# (see the gnu make manual section about automatic variables)
.cpp.o:
	$(CXX) $(CXXFLAGS) $(INCLUDES) -c -MMD $<  -o $@

.PHONY: clean
clean:
	$(RM) $(OUTPUTMAIN)
	$(RM) $(call FIXPATH,$(OBJECTS))
	$(RM) $(call FIXPATH,$(DEPS))
	$(RM) -r $(OUTPUT)
	@echo Cleanup complete!

run: all
	./$(OUTPUTMAIN)
	@echo Executing 'run: all' complete!
