cmake_minimum_required(VERSION 3.0)
project(LibAudioInput)
add_library(audioinput
            STATIC
            audio.c
)

install(TARGETS audioinput DESTINATION ./bin)