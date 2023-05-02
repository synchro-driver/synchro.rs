#include <locale.h>

#ifdef HAVE_ALLOCA_H
#include <alloca.h>
#else
#include <stdlib.h>
#endif

#include <fcntl.h>
//#include <math.h>

#include <stdio.h>
#include <stdlib.h>

#ifndef _MSC_VER
#include <ctype.h>
#include <dirent.h>
#include <getopt.h>
#include <sys/ioctl.h>
#include <termios.h>
#include <unistd.h>
#endif

#ifdef _MSC_VER
#include "input/winscap.h"
#include <windows.h>
#define PATH_MAX 260
#define PACKAGE "cava"
#define _CRT_SECURE_NO_WARNINGS 1
#endif // _MSC_VER

#include <signal.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <time.h>

#include "core.h"
#include "config.h"

#include "debug.h"
#include "util.h"
#include "input/common.h"

#ifndef _MSC_VER
#include "input/alsa.h"
#include "input/fifo.h"
#include "input/portaudio.h"
#include "input/pulse.h"
#include "input/shmem.h"
#include "input/sndio.h"
#endif

#ifdef __GNUC__
#undef GCC_UNUSED
#define GCC_UNUSED __attribute__((unused))
#else
#define GCC_UNUSED /* nothing */
#endif

#ifdef ALSA
static bool is_loop_device_for_sure(const char *text) {
    const char *const LOOPBACK_DEVICE_PREFIX = "hw:Loopback,";
    return strncmp(text, LOOPBACK_DEVICE_PREFIX, strlen(LOOPBACK_DEVICE_PREFIX)) == 0;
}
#endif

// Both these functions should be called inside an event loop during the invocation
struct audio_out audio_fetch() {
    struct audio_data audio;
    struct config_params p;
    struct audio_out audio_buffer;

    memset(&audio, 0, sizeof(audio));

    audio.source = malloc(1 + strlen(p.audio_source));
    strcpy(audio.source, p.audio_source);

    audio.format = -1;
    audio.rate = 0;
    audio.samples_counter = 0;
    audio.channels = 2;
    audio.IEEE_FLOAT = 0;

    audio.input_buffer_size = BUFFER_SIZE * audio.channels;
    audio.cava_buffer_size = audio.input_buffer_size * 8;

    audio.cava_in = (double *)malloc(audio.cava_buffer_size * sizeof(double));
    memset(audio.cava_in, 0, sizeof(int) * audio.cava_buffer_size);

    audio.terminate = 0;

    debug("starting audio thread\n");

    pthread_t p_thread;
    int timeout_counter = 0;

    struct timespec timeout_timer = {.tv_sec = 0, .tv_nsec = 1000000};
    int thr_id GCC_UNUSED;

    pthread_mutex_init(&audio.lock, NULL);

    switch (p.input) {
#ifndef _MSC_VER
#ifdef ALSA
        case INPUT_ALSA:
            if (is_loop_device_for_sure(audio.source)) {
                if (directory_exists("/sys/")) {
                    if (!directory_exists("/sys/module/snd_aloop/")) {
                        cleanup();
                        fprintf(stderr,
                                "Linux kernel module \"snd_aloop\" does not seem to  be loaded.\n"
                                "Maybe run \"sudo modprobe snd_aloop\".\n");
                        exit(EXIT_FAILURE);
                    }
                }
            }

            thr_id = pthread_create(&p_thread, NULL, input_alsa, (void *)&audio);
            break;
#endif
        case INPUT_FIFO:
            audio.rate = p.fifoSample;
            audio.format = p.fifoSampleBits;
            thr_id = pthread_create(&p_thread, NULL, input_fifo, (void *)&audio);
            break;
#ifdef PULSE
        case INPUT_PULSE:
            audio.format = 16;
            audio.rate = 44100;
            if (strcmp(audio.source, "auto") == 0) {
                getPulseDefaultSink((void *)&audio);
            }
            thr_id = pthread_create(&p_thread, NULL, input_pulse, (void *)&audio);
            break;
#endif
#ifdef SNDIO
        case INPUT_SNDIO:
            audio.format = 16;
            audio.rate = 44100;
            thr_id = pthread_create(&p_thread, NULL, input_sndio, (void *)&audio);
            break;
#endif
        case INPUT_SHMEM:
            audio.format = 16;
            thr_id = pthread_create(&p_thread, NULL, input_shmem, (void *)&audio);
            break;
#ifdef PORTAUDIO
        case INPUT_PORTAUDIO:
            audio.format = 16;
            audio.rate = 44100;
            thr_id = pthread_create(&p_thread, NULL, input_portaudio, (void *)&audio);
            break;
#endif
#endif
#ifdef _MSC_VER
        case INPUT_WINSCAP:
            thr_id = pthread_create(&p_thread, NULL, input_winscap, (void *)&audio);
            break;
#endif
        default:
            exit(EXIT_FAILURE); // Can't happen.
        }

        timeout_counter = 0;
    while (true) {
#ifdef _MSC_VER
        Sleep(1);
#else
        nanosleep(&timeout_timer, NULL);
#endif
        pthread_mutex_lock(&audio.lock);
        if (audio.format != -1 && audio.rate != 0)
            break;

        pthread_mutex_unlock(&audio.lock);
        timeout_counter++;
        if (timeout_counter > 2000) {
            cleanup();
            fprintf(stderr, "could not get rate and/or format, problems with audio thread? "
                            "quiting...\n");
            exit(EXIT_FAILURE);
        }
    }
    pthread_mutex_unlock(&audio.lock);
    debug("got format: %d and rate %d\n", audio.format, audio.rate);

    audio_buffer.stream = (double *) malloc(audio.input_buffer_size * sizeof(double));
    for(int i = 0; i < audio.input_buffer_size; i++)
        audio_buffer.stream[i] = audio.cava_in[i];

    audio_buffer.format = audio.format;
    audio_buffer.rate = audio.rate;
    audio_buffer.channels = audio_buffer.channels;
    audio_buffer.source = (char *) malloc(128 * sizeof(char));
    audio_buffer.sample_counter = audio.samples_counter;

    while(audio.source != NULL)
        audio_buffer.source = audio.source;

    audio_buffer.im = audio_buffer.im;
    for(int i = 0; i < 1024; i++)
    audio_buffer.error_message[i] = audio.error_message[i];

    return audio_buffer;
}
