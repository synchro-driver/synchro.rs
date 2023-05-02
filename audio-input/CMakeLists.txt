cmake_minimum_required(VERSION 3.0)
project(audioinput)
add_subdirectory(input)

add_library(
            ${PROJECT_NAME}
            STATIC
            audio.c
)

target_link_libraries(${PROJECT_NAME} PRIVATE audio-dependencies)

install(TARGETS audioinput DESTINATION ./bin)