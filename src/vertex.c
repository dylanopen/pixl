#include "pixl.h"

PixlVertexBuffer pixl_create_vertex_buffer_from_raw(int number_of_vertices, int floats_per_vertex, float raw_vertex_data[]) {
    GLuint vbo = 0;
    glad_glGenBuffers(1, &vbo);
    glad_glBindBuffer(GL_ARRAY_BUFFER, vbo);
    glad_glBufferData(GL_ARRAY_BUFFER, number_of_vertices * sizeof(float), raw_vertex_data, GL_STATIC_DRAW);

    GLuint vao = 0;
    glad_glGenVertexArrays(1, &vao);
    glad_glBindVertexArray(vao);
    glad_glEnableVertexAttribArray(0);
    glad_glBindBuffer(GL_ARRAY_BUFFER, vbo);
    glad_glVertexAttribPointer(0, floats_per_vertex, GL_FLOAT, GL_FALSE, 0, NULL);

    return (PixlVertexBuffer) {
	.vao = vao,
	.vbo = vbo,
    };
}

