cmake_minimum_required (VERSION 3.8)
project ("OpenVegasPro")

# Enable Hot Reload for MSVC compilers if supported.
if (POLICY CMP0141)
  cmake_policy(SET CMP0141 NEW)
  set(CMAKE_MSVC_DEBUG_INFORMATION_FORMAT "$<IF:$<AND:$<C_COMPILER_ID:MSVC>,$<CXX_COMPILER_ID:MSVC>>,$<$<CONFIG:Debug,RelWithDebInfo>:EditAndContinue>,$<$<CONFIG:Debug,RelWithDebInfo>:ProgramDatabase>>")
endif()

# Gather all .cpp files from the src folder
file(GLOB SOURCE_FILES "src/*.cpp")

# Add header files manually (if necessary)
set(HEADER_FILES 
    "src/enums.h"
    "src/utils.h"
)

# Add source to this project's executable.
add_executable (OpenVegasPro ${SOURCE_FILES} ${HEADER_FILES})

if (CMAKE_VERSION VERSION_GREATER 3.12)
  set_property(TARGET OpenVegasPro PROPERTY CXX_STANDARD 20)
endif()

# Find OpenCV package
find_package(OpenCV REQUIRED)

# Add external folder after OpenCV is found
add_subdirectory("external")
target_link_libraries(OpenVegasPro PRIVATE external ${OpenCV_LIBS} ${OpenCV_INCLUDE_DIRS})
