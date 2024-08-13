# CMakeList.txt : CMake project for OpenVegasPro, include source and define
# project specific logic here.
#
cmake_minimum_required (VERSION 3.8)

# Enable Hot Reload for MSVC compilers if supported.
if (POLICY CMP0141)
  cmake_policy(SET CMP0141 NEW)
  set(CMAKE_MSVC_DEBUG_INFORMATION_FORMAT "$<IF:$<AND:$<C_COMPILER_ID:MSVC>,$<CXX_COMPILER_ID:MSVC>>,$<$<CONFIG:Debug,RelWithDebInfo>:EditAndContinue>,$<$<CONFIG:Debug,RelWithDebInfo>:ProgramDatabase>>")
endif()

project ("OpenVegasPro")

# Setup Qt
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_PREFIX_PATH "C:\\Qt\\6.7.2\\msvc2019_64")

find_package(Qt6 COMPONENTS Widgets REQUIRED)



# Add source to this project's executable.
add_executable (OpenVegasPro "OpenVegasPro.cpp" "OpenVegasPro.h")

if (CMAKE_VERSION VERSION_GREATER 3.12)
  set_property(TARGET OpenVegasPro PROPERTY CXX_STANDARD 20)
endif()

# TODO: Add tests and install targets if needed.
target_link_libraries(OpenVegasPro Qt6::Widgets)
