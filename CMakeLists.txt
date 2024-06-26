cmake_minimum_required(VERSION 3.13)
project(baby-smash
        VERSION 1.0.1
        DESCRIPTION "Baby Smash"
        HOMEPAGE_URL https://github.com/axle-h/baby-smash)

set(CMAKE_CXX_STANDARD 17)

# Only support packaging, not installing from source.
set(CMAKE_INSTALL_PREFIX "${CMAKE_BINARY_DIR}")

configure_file (
    "${PROJECT_SOURCE_DIR}/src/config/BuildMeta.h.in"
    "${PROJECT_SOURCE_DIR}/src/config/BuildMeta.h"
)

include(FetchContent)
include(FindPkgConfig)

# Find required sdl2 libraries
pkg_check_modules(SDL2 REQUIRED IMPORTED_TARGET
        SDL2
        SDL2_image>=2.0.0
        SDL2_gfx)

# Find required libav libraries
pkg_check_modules(LIBAV REQUIRED IMPORTED_TARGET
        libavformat
        libavcodec
        libavutil
        libswscale
        libswresample)

# Fetch Box2D library from github
FetchContent_Declare(box2d
        GIT_REPOSITORY https://github.com/erincatto/box2d
        GIT_TAG v2.4.1)
FetchContent_MakeAvailable(box2d)

# Fetch json library from github
FetchContent_Declare(json
        GIT_REPOSITORY https://github.com/ArthurSonzogni/nlohmann_json_cmake_fetchcontent
        GIT_TAG v3.9.1)
FetchContent_MakeAvailable(json)

# Fetch spdlog library from github
FetchContent_Declare(spdlog
        GIT_REPOSITORY https://github.com/gabime/spdlog
        GIT_TAG v1.x)
FetchContent_MakeAvailable(spdlog)

file(GLOB_RECURSE SOURCE_FILES
        "${PROJECT_SOURCE_DIR}/src/*.h"
        "${PROJECT_SOURCE_DIR}/src/*.cpp"
        "${PROJECT_SOURCE_DIR}/src/*.c")

# TODO move into resources module
function(resource VAR SOURCE_PATH DESTINATION PATTERN)
    if (APPLE)
        set(_PREFIX Resources)
    else()
        set(_PREFIX ".")
    endif()
    file(GLOB_RECURSE _LIST CONFIGURE_DEPENDS ${SOURCE_PATH}/${PATTERN})
    list(FILTER _LIST EXCLUDE REGEX [[^\.]])
    foreach (RESOURCE ${_LIST})
        get_filename_component(_PARENT ${RESOURCE} DIRECTORY)
        if (${_PARENT} STREQUAL ${SOURCE_PATH})
            set(_DESTINATION ${_PREFIX}/${DESTINATION})
        else ()
            file(RELATIVE_PATH _DESTINATION ${SOURCE_PATH} ${_PARENT})
            set(_DESTINATION ${_PREFIX}/${DESTINATION}/${_DESTINATION})
        endif ()
        if (APPLE)
            set_property(SOURCE ${RESOURCE} PROPERTY MACOSX_PACKAGE_LOCATION ${_DESTINATION})
        elseif (WIN32)
            file(COPY ${RESOURCE} DESTINATION ${_DESTINATION})
            install(FILES ${RESOURCE} DESTINATION ${_DESTINATION} COMPONENT app)
        else()
            file(COPY ${RESOURCE} DESTINATION ${_DESTINATION})
            install(FILES ${RESOURCE} DESTINATION axle-h/${CMAKE_PROJECT_NAME}/${_DESTINATION} COMPONENT app)
        endif()

    endforeach (RESOURCE)
    set(${VAR} ${_LIST} PARENT_SCOPE)
endfunction()

set(RESOURCES_PATH "${PROJECT_SOURCE_DIR}/assets")

if (APPLE)
    resource(MACOS_ICON_PATH ${RESOURCES_PATH} "" ${CMAKE_PROJECT_NAME}.icns)
endif()

resource(AUDIO_CREATE_FILES ${RESOURCES_PATH}/audio/create audio/create *.*)
resource(AUDIO_DESTROY_FILES ${RESOURCES_PATH}/audio/destroy audio/destroy *.*)
resource(AUDIO_MUSIC_FILES ${RESOURCES_PATH}/audio/music audio/music *.*)
resource(VIDEO_FILES ${RESOURCES_PATH}/video video *.*)
resource(SPRITE_FILES ${RESOURCES_PATH} "" sprites.*)
set(RESOURCE_FILES
    ${AUDIO_CREATE_FILES}
    ${AUDIO_DESTROY_FILES}
    ${AUDIO_MUSIC_FILES}
    ${VIDEO_FILES}
    ${SPRITE_FILES}
    ${MACOS_ICON_PATH})

add_executable(${CMAKE_PROJECT_NAME} ${SOURCE_FILES} ${RESOURCE_FILES})

target_link_libraries(${CMAKE_PROJECT_NAME}
        PkgConfig::LIBAV
        PkgConfig::SDL2
        box2d
        nlohmann_json::nlohmann_json
        spdlog)

if (APPLE)
    target_link_libraries(${CMAKE_PROJECT_NAME}
            "-framework CoreGraphics"
            "-framework Foundation"
            "-framework QuartzCore")

    install(TARGETS ${CMAKE_PROJECT_NAME} DESTINATION . COMPONENT app)
    set_target_properties(${CMAKE_PROJECT_NAME} PROPERTIES
            MACOSX_BUNDLE TRUE
            MACOSX_BUNDLE_BUNDLE_NAME "${PROJECT_DESCRIPTION}"
            MACOSX_BUNDLE_COPYRIGHT "Alex Haslehurst"
            MACOSX_BUNDLE_BUNDLE_VERSION ${PROJECT_VERSION}
            MACOSX_BUNDLE_INFO_STRING "${PROJECT_DESCRIPTION} ${PROJECT_VERSION}"
            MACOSX_BUNDLE_LONG_VERSION_STRING ${PROJECT_VERSION}
            MACOSX_BUNDLE_SHORT_VERSION_STRING ${PROJECT_VERSION}
            MACOSX_BUNDLE_ICON_FILE ${CMAKE_PROJECT_NAME})

    install(CODE [[
            include(BundleUtilities)
            fixup_bundle(${CMAKE_INSTALL_PREFIX}/baby-smash.app "" "")
    ]] COMPONENT app)

    set(CPACK_GENERATOR "ZIP")
    set(CPACK_ARCHIVE_COMPONENT_INSTALL ON)
    set(CPACK_COMPONENTS_ALL app)
elseif (WIN32)
    install(TARGETS ${CMAKE_PROJECT_NAME} DESTINATION . COMPONENT app)
    install(CODE [[
            include(BundleUtilities)
            fixup_bundle(${CMAKE_INSTALL_PREFIX}/baby-smash.exe "" "")
    ]] COMPONENT app)

    set(CPACK_GENERATOR "ZIP")
    set(CPACK_ARCHIVE_COMPONENT_INSTALL ON)
    set(CPACK_COMPONENTS_ALL app)
elseif (UNIX)
    install(TARGETS ${CMAKE_PROJECT_NAME} DESTINATION share/axle-h/${CMAKE_PROJECT_NAME} COMPONENT app)
    install(FILES ${RESOURCES_PATH}/${CMAKE_PROJECT_NAME}.desktop DESTINATION share/applications COMPONENT app)
    install(FILES ${RESOURCES_PATH}/${CMAKE_PROJECT_NAME}.png DESTINATION share/pixmaps COMPONENT app)
    set(CPACK_GENERATOR "DEB")
    set(CPACK_DEB_COMPONENT_INSTALL ON)
    set(CPACK_COMPONENTS_ALL app)
    set(CPACK_DEBIAN_PACKAGE_SHLIBDEPS ON)
    set(CPACK_DEBIAN_PACKAGE_DESCRIPTION ${PROJECT_DESCRIPTION})
    set(CPACK_DEBIAN_PACKAGE_VERSION ${PROJECT_VERSION})
    set(CPACK_DEBIAN_PACKAGE_NAME ${PROJECT_NAME})
    set(CPACK_DEBIAN_APP_PACKAGE_NAME ${PROJECT_NAME})
    set(CPACK_DEBIAN_PACKAGE_MAINTAINER "Alex Haslehurst")
endif()

include(CPack)
