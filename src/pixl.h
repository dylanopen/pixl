#ifndef PIXL_H
#define PIXL_H

#include "lib/gl.h"
#include <stdint.h>
#include <stdio.h>
#include "lib/result.h"
#include "lib/glfw3.h"
#include "lib/result.h"


//--- Window management, window.c ---//

typedef struct {
    GLFWwindow *glfwWindow;
} PixlWindow;

typedef enum {
    PIXL_CREATE_WINDOW_ERROR_GLFW_INIT,
    PIXL_CREATE_WINDOW_ERROR_GLFW_CREATE_WINDOW,
} PixlCreateWindowError;

RESULT_STRUCT(PixlWindow, PixlCreateWindowError);

RESULT(PixlWindow, PixlCreateWindowError) pixl_create_window(uint32_t width, uint32_t height, char* title);


//--- Colors, color.c ---//

typedef struct {
    uint8_t r;
    uint8_t g;
    uint8_t b;
    uint8_t a;
} PixlColor;

PixlColor pixl_color_rgba(uint8_t r, uint8_t g, uint8_t b, uint8_t a);
PixlColor pixl_color_rgb(uint8_t r, uint8_t g, uint8_t b);


//--- Vertex OpenGL storage, vertex.c ---//

typedef struct {
    GLuint vao;
    GLuint vbo;
} VertexBuffer;

VertexBuffer pixl_create_vertex_buffer_from_raw(int number_of_vertices, int floats_per_vertex, float raw_vertex_data[]);


#endif
