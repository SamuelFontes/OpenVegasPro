# add folders
add_subdirectory(raylib)

# define interface
add_library(external INTERFACE)

# Add libraries to interface
target_link_libraries(external INTERFACE raylib)
target_include_directories(external INTERFACE "raygui/src/" "header")