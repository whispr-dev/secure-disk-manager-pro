cmake_minimum_required(VERSION 3.16)
project(SecureDiskManagerPro)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Root include paths (case corrected)
include_directories(
    ${PROJECT_SOURCE_DIR}/Core
    ${PROJECT_SOURCE_DIR}/CLI
    ${PROJECT_SOURCE_DIR}/identity
    ${PROJECT_SOURCE_DIR}/GhostMode_Vanisher/include
    ${PROJECT_SOURCE_DIR}/GhostMode_Vanisher/plugins
    ${PROJECT_SOURCE_DIR}/GhostMode_Vanisher/net
)

# Gather source files (case corrected)
file(GLOB_RECURSE CORE_SRC         ${PROJECT_SOURCE_DIR}/Core/*.cpp)
file(GLOB_RECURSE CLI_SRC          ${PROJECT_SOURCE_DIR}/CLI/*.cpp)
file(GLOB_RECURSE IDENTITY_SRC     ${PROJECT_SOURCE_DIR}/identity/*.cpp)
file(GLOB_RECURSE GHOST_SRC        ${PROJECT_SOURCE_DIR}/GhostMode_Vanisher/src/*.cpp)
file(GLOB_RECURSE GHOST_NET_SRC    ${PROJECT_SOURCE_DIR}/GhostMode_Vanisher/net/*.cpp)
file(GLOB_RECURSE GHOST_PLUGIN_SRC ${PROJECT_SOURCE_DIR}/GhostMode_Vanisher/plugins/*.cpp)

# Collect everything
set(ALL_SRC
    ${CORE_SRC}
    ${CLI_SRC}
    ${IDENTITY_SRC}
    ${GHOST_SRC}
    ${GHOST_NET_SRC}
    ${GHOST_PLUGIN_SRC}
)

# Main executable
add_executable(SecureDiskManagerPro ${ALL_SRC})

# OS specific libs
if (WIN32)
    target_link_libraries(SecureDiskManagerPro ws2_32)
elseif(UNIX)
    find_package(Threads REQUIRED)
    target_link_libraries(SecureDiskManagerPro Threads::Threads)
endif()
