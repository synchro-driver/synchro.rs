cmake_minimum_required(VERSION 3.0)
project(audioinput)
add_subdirectory(input)

add_library(
            ${PROJECT_NAME}
            STATIC
            audio.c
)

target_link_libraries(${PROJECT_NAME} PUBLIC audio-dependencies)

install(TARGETS ${PROJECT_NAME} DESTINATION /home/cyberflaw/code/user/synchro.rs/audio-input/bin)